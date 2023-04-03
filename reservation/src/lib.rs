mod manager;

pub use abi::*;
pub use async_trait::async_trait;
pub use sqlx::PgPool;
use tokio::sync::mpsc;

/** 包裹一个 PgPool，以便在一个应用程序中分发共享数据库链接 */
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    // make a reservation
    // 构建预定
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, Error>;
    // change reservation status, if current status is pending, change to confirm
    // 更新预定,如果当前状态时`待定`的，则更新为确认
    async fn update_status(&self, id: ReservationId) -> Result<abi::Reservation, Error>;
    // update reservation note
    async fn update_note(&self, id: ReservationId, note: String)
        -> Result<abi::Reservation, Error>;
    // delete reservation by id
    async fn delete(&self, id: ReservationId) -> Result<abi::Reservation, Error>;
    // get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, Error>;
    // query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, Error>>;
    // To query reservation order by id
    async fn filter(
        &self,
        filter: ReservationFilter,
    ) -> Result<(FilterPager, Vec<abi::Reservation>), Error>;
}
