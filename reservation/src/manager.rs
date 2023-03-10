use crate::{error, ReservationId, ReservationManager, Rsvp};
use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgRange, Row};

#[warn(non_snake_case)]
#[async_trait::async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, error::ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(error::ReservationError::InvalidTime);
        };

        let start: NaiveDateTime =
            NaiveDateTime::from_timestamp_opt(rsvp.start.clone().unwrap().seconds, 0).unwrap();
        let end: NaiveDateTime =
            NaiveDateTime::from_timestamp_opt(rsvp.end.clone().unwrap().seconds, 0).unwrap();
        if start <= end {
            return Err(error::ReservationError::InvalidTime);
        };

        let timespan: PgRange<_> = (start..end).into();
        let id = sqlx::query(
            "INSERT INTO reservation(user_id, resource_id, timespan, note ,status) VALUES($1, $2, $3, $4,   $5) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(rsvp.status)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn update_status(
        &self,
        id: ReservationId,
    ) -> Result<abi::Reservation, error::ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        id: ReservationId,
    ) -> Result<abi::Reservation, error::ReservationError> {
        todo!()
    }

    async fn delete(&self, id: ReservationId) -> Result<(), error::ReservationError> {
        todo!()
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, error::ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, error::ReservationError> {
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
    use crate::{ReservationManager, Rsvp};
    use abi::convert_str_to_Timestamp;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_volid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp = abi::Reservation {
            id: "".to_string(),
            user_id: "zz id".to_string(),
            resource_id: "ocean-view-room-713".to_string(),
            start: Some(convert_str_to_Timestamp("2023-03-10 17:20:35")),
            end: Some(convert_str_to_Timestamp("2023-03-11 17:20:35")),
            // end: NaiveDateTime::parse_from_str("2023-03-11 16:53:56", &fmt).unwrap(),
            note: "我明天晚上7点入住".to_string(),
            status: abi::ReservationStatus::Pending as i32,
        };

        let rsvp = manager.reserve(rsvp).await.unwrap();
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
