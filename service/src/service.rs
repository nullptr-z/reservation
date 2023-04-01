use std::{pin::Pin, task::Poll};

use abi::{reservation_service_server::ReservationService, *};
use futures::Stream;
use reservation::{ReservationManager, Rsvp};
use tokio::sync::mpsc;
use tonic::*;

use crate::{ReservationStream, RsvpService, TonicReceiverStream};

impl RsvpService {
    pub async fn from_config(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            manager: ReservationManager::from_config(&config.db).await?,
        })
    }
}

impl<T> TonicReceiverStream<T> {
    pub fn new(inner: mpsc::Receiver<Result<T, Error>>) -> Self {
        Self { inner }
    }
}

impl<T> Stream for TonicReceiverStream<T> {
    type Item = Result<T, Status>;

    fn poll_next(
        mut self: Pin<&mut TonicReceiverStream<T>>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.inner.poll_recv(cx) {
            Poll::Ready(Some(Ok(item))) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[async_trait]
impl ReservationService for RsvpService {
    /// make a reservation
    /// 构建一个预定
    async fn reserve(
        &self,
        request: Request<ReserveRequest>,
    ) -> Result<Response<ReserveResponse>, Status> {
        let request = request.into_inner();

        if request.reservation.is_none() {
            return Err(Status::invalid_argument("missing reservation"));
        }

        let reservation = self.manager.reserve(request.reservation.unwrap()).await?;
        Ok(Response::new(ReserveResponse {
            reservation: Some(reservation),
        }))
    }
    /// confirm a pending reservation, if reservation is not pending, do nothing
    /// 确认待定的预定，如果预定不是待定的，什么都不做
    async fn confirm(
        &self,
        request: Request<ConfirmRequest>,
    ) -> Result<Response<ConfirmResponse>, Status> {
        let request = request.into_inner();
        let reserevation = self.manager.update_status(request.id).await?;
        Ok(Response::new(ConfirmResponse {
            reservation: Some(reserevation),
        }))
    }
    /// update the reservation note
    /// 更新这个预定的注释`note`
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        // let request = request.into_inner();
        // let reservation = self.manager.update(request.id);
        todo!()
    }
    /// cancle a reservation
    /// 取消预定
    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.delete(request.id).await?;
        Ok(Response::new(CancelResponse {
            reservation: Some(reservation),
        }))
    }
    /// get a reservation by id
    /// 根据ID获取预定
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.get(request.id).await?;
        Ok(Response::new(GetResponse {
            reservation: Some(reservation),
        }))
    }
    /// Server streaming response type for the Query method.
    type QueryStream = ReservationStream;
    /// query reservation by resource id, user id, status, start time, end time
    /// 通过资源id、用户id、状态、开始时间、结束时间 查询预约
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        let request = request.into_inner();
        if request.query.is_none() {
            return Err(Status::invalid_argument("missing query params"));
        }
        let rsvps = self.manager.query(request.query.unwrap()).await;
        let stream = TonicReceiverStream::new(rsvps);
        Ok(Response::new(Box::pin(stream)))
    }
    /// Server streaming response type for the filter method.
    /// 查询时通过ID进行排序
    async fn filter(
        &self,
        request: Request<FilterRequest>,
    ) -> Result<Response<FilterResponse>, Status> {
        let request = request.into_inner();

        if request.filter.is_none() {
            return Err(Status::invalid_argument("missing filter params"));
        }

        let (pager, reservations) = self.manager.filter(request.filter.unwrap()).await?;
        Ok(Response::new(FilterResponse {
            pager: Some(pager),
            reservations,
        }))
    }
    /// Server streaming response type for the listen method.
    type listenStream = ReservationStream;
    /// an other system could moniton newly added/confirmed/cancelled reservation
    /// 另一个系统可以监控新添加/确认/取消的预订
    async fn listen(
        &self,
        request: Request<ListenRequest>,
    ) -> Result<Response<Self::listenStream>, Status> {
        todo!()
    }
}

mod tests {
    use std::ops::Deref;

    use crate::RsvpService;
    use abi::{
        reservation_service_server::ReservationService, Config, Reservation, ReserveRequest,
    };
    use db_sqlx_tester::TestDb;

    struct TestConfig {
        #[allow(dead_code)]
        tdb: TestDb,
        config: Config,
    }

    impl TestConfig {
        pub fn new() -> Self {
            let mut config = Config::load("fixtures/config.yml").unwrap();
            let tdb = TestDb::new(
                &config.db.host,
                config.db.port,
                &config.db.user,
                &config.db.password,
                "../migrations",
            );
            config.db.dbname = tdb.dbname.clone();
            Self { tdb, config }
        }
    }

    impl Deref for TestConfig {
        type Target = Config;

        fn deref(&self) -> &Self::Target {
            &self.config
        }
    }

    async fn rpc_reserve_should_work() {
        let mut config = TestConfig::new();

        let service = RsvpService::from_config(&config).await.unwrap();
        let reservation = Reservation::new_pending(
            "zz id",
            "xierdun-1101",
            "2023-03-23 22:44:28",
            "2023-03-25 22:44:28",
            "waiting for you",
        );
        let request = tonic::Request::new(ReserveRequest {
            reservation: Some(reservation.clone()),
        });
        let response = service.reserve(request).await.unwrap();
        let reservation1 = response.into_inner().reservation;
        assert!(reservation1.is_some());
        let reservation1 = reservation1.unwrap();
        assert_eq!(reservation1.user_id, reservation.user_id);
        assert_eq!(reservation1.resource_id, reservation.resource_id);
        assert_eq!(reservation1.start, reservation.start);
        assert_eq!(reservation1.end, reservation.end);
        assert_eq!(reservation1.note, reservation.note);
        assert_eq!(reservation1.status, reservation.status);
        drop(service);
    }
}
