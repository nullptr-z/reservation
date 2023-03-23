use abi::{reservation_service_server::ReservationService, *};
use anyhow::Result;
use reservation::{ReservationManager, Rsvp};
use tonic::*;

use crate::{FilterResponseStream, ReservationStream, RsvpService};

impl RsvpService {
    pub async fn from_config(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            manager: ReservationManager::from_config(&config.db).await?,
        })
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
        todo!()
    }
    /// update the reservation note
    /// 更新这个预定的注释`note`
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        todo!()
    }
    /// cancle a reservation
    /// 取消预定
    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        todo!()
    }
    /// get a reservation by id
    /// 根据ID获取预定
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }
    /// Server streaming response type for the Query method.
    type QueryStream = ReservationStream;
    /// query reservation by resource id, user id, status, start time, end time
    /// 通过资源id、用户id、状态、开始时间、结束时间 查询预约
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        todo!()
    }
    /// Server streaming response type for the filter method.
    type filterStream = FilterResponseStream;
    /// 查询时通过ID进行排序
    async fn filter(
        &self,
        request: Request<FilterRequest>,
    ) -> Result<Response<Self::filterStream>, Status> {
        todo!()
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
    use abi::{
        reservation_service_server::ReservationService, Config, Reservation, ReserveRequest,
    };

    use crate::RsvpService;

    #[tokio::test]
    async fn rpc_reserve_should_work() {
        let config = Config::load("../server/fixtures/config.json").unwrap();
        let service = RsvpService::from_config(&config).await.unwrap();
        let reservation = Reservation::new_pending(
            "zz id",
            "xierdun-1101",
            "2023-03-23 22:44:28",
            "2023-03-25 22:44:28",
            "waiting for you",
        );
        let request = tonic::Request::new(ReserveRequest {
            reservation: Some(reservation),
        });
        let response = service.reserve(request).await.unwrap();
        assert!(response.into_inner().reservation.is_some());
    }
}
