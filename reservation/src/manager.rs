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
}
