use crate::{ReservationId, ReservationManager, Rsvp};
use abi::{Error, ReservationStatus};
use chrono::NaiveDateTime;
use sqlx::{types::Uuid, Row};

#[async_trait::async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, Error> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(Error::InvalidTime);
        };

        let start: NaiveDateTime =
            NaiveDateTime::from_timestamp_opt(rsvp.start.clone().unwrap().seconds, 0).unwrap();
        let end: NaiveDateTime =
            NaiveDateTime::from_timestamp_opt(rsvp.end.clone().unwrap().seconds, 0).unwrap();
        if end <= start {
            return Err(Error::InvalidTime);
        };
        let timespan = format!("[{}, {})", start, end);

        let status = ReservationStatus::from_i32(rsvp.status).unwrap_or(ReservationStatus::Pending);

        let id:Uuid = sqlx::query(
            "INSERT INTO rsvp.reservation (user_id, resource_id, timespan, note ,status) VALUES ($1, $2, $3::tstzrange, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id.to_string();

        Ok(rsvp)
    }

    async fn update_status(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        todo!()
    }

    async fn update_note(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        todo!()
    }

    async fn delete(&self, id: ReservationId) -> Result<(), Error> {
        todo!()
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        todo!()
    }

    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, Error> {
        todo!()
    }
}

impl ReservationManager {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_volid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp = abi::Reservation::new_pending(
            "zz id",
            "ocean-view-room-713",
            "2023-03-10 17:20:35",
            "2023-03-12 17:20:35",
            "i'm today evening to check in",
        );

        let rsvp = manager.reserve(rsvp).await.expect("run reserve error");
        assert_ne!(rsvp.id, "");
    }

    // #[tokio::test]
    // async fn test_reserve() {
    //     // 创建一个新的 ReservationManager 实例并使用测试数据库的连接池
    //     // 设置一个用于测试的 PostgreSQL 数据库和使用测试数据库的连接池。
    //     let pool = sqlx::Pool::builder()
    //         .max_size(1)
    //         .build(&std::env::var("DATABASE_URL").unwrap())
    //         .await
    //         .unwrap();

    //     // 使用测试池创建一个新的 ReservationManager。
    //     let reservation_manager = ReservationManager::new(pool);

    //     // 创建一个新的 Reservation，将其字段设置为合适的值
    //     let user_id = "test_user".to_string();
    //     let resource_id = "test_resource".to_string();
    //     let now = Utc::now();
    //     let start = now + Duration::minutes(30);
    //     let end = now + Duration::hours(1);
    //     let start_timestamp = start
    //         .timestamp()
    //         .try_into()
    //         .expect("Failed to convert start time to i64");
    //     let end_timestamp = end
    //         .timestamp()
    //         .try_into()
    //         .expect("Failed to convert end time to i64");
    //     let reservation = abi::Reservation {
    //         id: String::new(),
    //         user_id: user_id.clone(),
    //         resource_id: resource_id.clone(),
    //         start: Some(prost_types::Timestamp::from_i64(start_timestamp)),
    //         end: Some(prost_types::Timestamp::from_i64(end_timestamp)),
    //         note: Some(note.clone()),
    //         status: abi::ReservationStatus::Confirmed as i32,
    //     };

    //     // 调用 reserve 函数并检查返回结果是否为 Ok
    //     let result = reservation_manager.reserve(reservation.clone()).await;
    //     assert!(result.is_ok());

    //     // 检查返回的 Reservation 是否与输入的 Reservation 匹配
    //     let returned_reservation = result.unwrap();
    //     assert_eq!(returned_reservation.user_id, user_id);
    //     assert_eq!(returned_reservation.resource_id, resource_id);
    //     assert_eq!(returned_reservation.start.unwrap(), start.into());
    //     assert_eq!(returned_reservation.end.unwrap(), end.into());
    //     assert_eq!(returned_reservation.note.unwrap(), note);
    //     assert_eq!(
    //         returned_reservation.status,
    //         abi::ReservationStatus::Confirmed as i32
    //     );

    //     // 检查在数据库中是否成功创建了这个 Reservation
    //     let query = query_scalar(
    //         "SELECT COUNT(*) FROM reservation WHERE user_id = $1 AND resource_id = $2 AND timespan = $3 AND note = $4 AND status = $5",
    //     )
    //     .bind(user_id)
    //     .bind(resource_id)
    //     .bind(format!("[{}, {})", start.timestamp(), end.timestamp()))
    //     .bind(note)
    //     .bind(abi::ReservationStatus::Confirmed as i32)
    //     .fetch_one(&reservation_manager.pool)
    //     .await
    //     .unwrap();

    //     assert_eq!(query, 1);
    // }
}
