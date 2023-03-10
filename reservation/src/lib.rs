mod error;
mod manager;

use async_trait::async_trait;
use error::*;
use sqlx::PgPool;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

pub struct ReservationManager {
    // postgres，可以在一个应用程序中分发共享数据库链接
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    // make a reservation
    // 构建预定
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError>;
    // change reservation status, if current status is pending, change to confirm
    // 更新预定,如果当前状态时`待定`的，则更新为确认
    async fn update_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    // update reservation note
    async fn update_note(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    // delete reservation by id
    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError>;
    // get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    // query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError>;
}
