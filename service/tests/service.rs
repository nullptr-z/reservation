#[path = "../src/test_utils.rs"]
mod test_utils;

use crate::test_utils::TestConfig;
use abi::{
    reservation_service_client::ReservationServiceClient, ConfirmRequest, FilterRequest,
    FilterResponse, Reservation, ReservationFilterBuilder, ReservationStatus, ReserveRequest,
};
use abi::{Config, QueryRequest, ReservationQueryBuilder};
use futures::StreamExt;
use reservation_service::start_server;
use std::time::Duration;
use tokio::time;
use tonic::transport::Channel;

#[tokio::test]
async fn grpc_server_should_work() {
    let tconfig = TestConfig::with_server_port(50000);
    let mut client = get_test_client(&tconfig).await;

    // first we make a reservation
    let reservation = Reservation::new_pending(
        "zz id",
        "Sunday haapy",
        "2023-04-01 12:00:00Z",
        "2023-04-08 12:00:00Z",
        "Sunday check in",
    );
    // then we   first reservation
    let rsvt = client
        .reserve(ReserveRequest::new(reservation.clone()))
        .await;
    assert_eq!(
        rsvt.unwrap_err().to_string(),
        "rpc error: code = InvalidArgument desc = reservation conflict"
    );

    // then we confirm first reservation
    let rsvt = client
        .confirm(ConfirmRequest::new(reservation.id))
        .await
        .unwrap()
        .into_inner()
        .reservation
        .unwrap();

    assert_eq!(rsvt.user_id, reservation.user_id);
}

#[tokio::test]
async fn grpc_query_should_work() {
    let tconfig = TestConfig::with_server_port(50002);
    let mut client = get_test_client(&tconfig).await;
    make_reservertions(&mut client, 100).await;

    let query = ReservationQueryBuilder::default()
        .user_id("zz id".to_string())
        .build()
        .unwrap();
    // query al reservation
    let mut rsvt = client
        .query(QueryRequest::new(query))
        .await
        .unwrap()
        .into_inner();

    while let Some(Ok(rsvp)) = rsvt.next().await {
        assert_eq!(rsvp.user_id, "zz id");
    }
}

#[tokio::test]
async fn grpc_filter_should_work() {
    let tconfig = TestConfig::with_server_port(50001);
    let mut client = get_test_client(&tconfig).await;

    make_reservertions(&mut client, 100).await;

    // then we filter by user
    let filter = ReservationFilterBuilder::default()
        .user_id("alice")
        .status(ReservationStatus::Pending as i32)
        .build()
        .unwrap();
    let FilterResponse {
        pager,
        reservations: _reservations,
    } = client
        .filter(FilterRequest::new(filter.clone()))
        .await
        .unwrap()
        .into_inner();

    let pager = pager.unwrap();
    assert_eq!(pager.next, filter.page_size);
    assert_eq!(pager.prev, -1);
    // assert_eq!(reservations.len(), filter.page_size as usize);
    ////////////////////////
    let mut next_filter = filter.clone();
    next_filter.cursor = pager.next;
    // then we get next page
    let FilterResponse {
        pager,
        reservations,
    } = client
        .filter(FilterRequest::new(next_filter.clone()))
        .await
        .unwrap()
        .into_inner();
    let pager = pager.unwrap();
    assert_eq!(pager.next, next_filter.cursor + filter.page_size);
    assert_eq!(pager.prev, next_filter.cursor - 1);
    assert_eq!(reservations.len(), filter.page_size as usize);
}

async fn get_test_client(tconfig: &TestConfig) -> ReservationServiceClient<Channel> {
    let config = tconfig.config.clone();
    setup_server(&config).await;

    let dst = config.server.url(false);
    let client = ReservationServiceClient::connect(dst).await.unwrap();

    client
}

async fn setup_server(config: &Config) {
    let config_cloned = config.clone();
    tokio::spawn(async move {
        start_server(&config_cloned).await.unwrap();
    });
    time::sleep(Duration::from_millis(100)).await;
}

/**  then we make 100 reservations without confliction
 *  创建 100 个预定
 */
async fn make_reservertions(client: &mut ReservationServiceClient<Channel>, count: u32) {
    for i in 0..count {
        let mut reservation = Reservation::new_pending(
            "alice",
            format!("router-{}", i),
            "2023-04-01 12:00:00Z",
            "2023-04-08 12:00:00Z",
            format!("Sunday check in-{}", i),
        );
        let rsvt = client
            .reserve(ReserveRequest::new(reservation.clone()))
            .await
            .unwrap()
            .into_inner()
            .reservation
            .unwrap();

        reservation.id = rsvt.id;
        assert_eq!(reservation, rsvt);
    }
}
