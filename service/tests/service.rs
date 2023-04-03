#[path = "../src/test_utils.rs"]
mod test_utils;

use std::time::Duration;

use abi::{
    reservation_service_client::ReservationServiceClient, ConfirmRequest, FilterRequest,
    FilterResponse, Reservation, ReservationFilterBuilder, ReservationStatus, ReserveRequest,
};
use reservation_service::start_server;
use tokio::time;
use tonic::transport::Channel;

use crate::test_utils::TestConfig;

#[tokio::test]
async fn grpc_server_should_work() {
    let tconfig = TestConfig::default();
    let config = tconfig.config.clone();
    let config_cloned = config.clone();
    tokio::spawn(async move {
        start_server(&config_cloned).await.unwrap();
    });
    time::sleep(Duration::from_millis(100)).await;
    let mut client = ReservationServiceClient::connect(config.server.url(false))
        .await
        .unwrap();
    // let tconfig = TestConfig::with_server_port(50000);
    // let mut client = get_test_client(&tconfig).await;

    // first we make a reservation
    let reservation = Reservation::new_pending(
        "zz id",
        "Sunday haapy",
        "2023-04-01 12:00:00Z",
        "2023-04-08 12:00:00Z",
        "Sunday check in",
    );
    // then we   first reservation
    let response = client
        .reserve(ReserveRequest::new(reservation.clone()))
        .await;
    assert_eq!(
        response.unwrap_err().to_string(),
        "rpc error: code = InvalidArgument desc = reservation conflict"
    );

    // then we confirm first reservation
    let response = client
        .confirm(ConfirmRequest::new(reservation.id))
        .await
        .unwrap()
        .into_inner()
        .reservation
        .unwrap();

    assert_eq!(response.user_id, reservation.user_id);

    // then we make 100 reservations without confliction
    for i in 0..100 {
        let reservation = Reservation::new_pending(
            "alice",
            format!("router-{}", i),
            "2023-04-01 12:00:00Z",
            "2023-04-08 12:00:00Z",
            format!("Sunday check in-{}", i),
        );
        let response = client
            .reserve(ReserveRequest::new(reservation.clone()))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.reservation, Some(reservation.clone()));
    }

    // then we filter by user
    let filter = ReservationFilterBuilder::default()
        .user_id("alice")
        .status(ReservationStatus::Pending as i32)
        .build()
        .unwrap();
    let FilterResponse {
        pager,
        reservations,
    } = client
        .filter(FilterRequest::new(filter.clone()))
        .await
        .unwrap()
        .into_inner();

    let pager = pager.unwrap();
    assert_eq!(pager.next, filter.page_size + 1 + 1);
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
    todo!()
}
