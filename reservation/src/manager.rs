use crate::{error, ReservationId, ReservationManager, Rsvp};
use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgRange, Row};

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
            "INSERT INTO reservation(user_id, resource_id, timespan, note ,status) VALUES($1,$2,$3,$4,$5) RETURNING id"
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
