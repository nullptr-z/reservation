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
    use crate::RsvpService;
    use abi::{
        reservation_service_server::ReservationService, Config, Reservation, ReserveRequest,
    };
    use lazy_static::lazy_static;
    use sqlx::{sqlx_macros::migrate, types::Uuid, Connection, Executor, PgConnection};
    use std::{sync::Arc, thread};
    use tokio::runtime::Runtime;

    lazy_static! {
        static ref TEST_RT: Runtime = Runtime::new().unwrap();
    }

    struct TestConfig {
        config: Arc<Config>,
    }

    impl std::ops::Deref for TestConfig {
        type Target = Config;

        fn deref(&self) -> &Self::Target {
            &self.config
        }
    }

    impl TestConfig {
        pub fn new() -> Self {
            let mut config = Config::load("../service/fixtures/config.yml").unwrap();

            let uuid = Uuid::new_v4();
            let dbname = format!("test_{}", uuid);
            config.db.dbname = dbname.clone();

            let server_url = config.db.server_url();
            let url = config.db.url();

            thread::spawn(move || {
                TEST_RT.block_on(async move {
                    // use server url to create database
                    let mut conn = PgConnection::connect(&server_url).await.unwrap();
                    conn.execute(format!(r#"CREATE DATABASE "{}""#, dbname).as_str())
                        .await
                        .expect("Error while querying the reservation database");

                    // now connect to test database for migration
                    let mut conn = PgConnection::connect(&url).await.unwrap();
                    migrate!("../migrations").run(&mut conn).await.unwrap();
                });
            })
            .join()
            .expect("Error thread create database ");

            Self {
                config: Arc::new(config),
            }
        }
    }

    impl Drop for TestConfig {
        fn drop(&mut self) {
            let server_url = self.config.db.server_url();
            let dbname = self.config.db.dbname.clone();
            // drop 时删除数据库
            thread::spawn(move || {
                TEST_RT.block_on(async move {
                    let mut conn = sqlx::PgConnection::connect(&server_url).await.unwrap();
                    // terminate existing connection`中断现有连接
                    sqlx::query(&format!(r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = '{}'"#,dbname))
                    .execute(&mut conn)
                    .await
                    .expect("Terminal all other connections");

                    conn.execute(format!(r#"DROP DATABASE "{}""#, dbname).as_str())
                        .await
                        .expect("Error while querying the reservation database");
                });
            })
            .join()
            .expect("Error thread drop database ");
        }
    }

    #[tokio::test]
    async fn rpc_reserve_should_work() {
        let config = TestConfig::new();
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
