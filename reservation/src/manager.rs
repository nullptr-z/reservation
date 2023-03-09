use chrono::{NaiveDate, NaiveDateTime, Utc};

use crate::{error, ReservationId, ReservationManager, Rsvp};

#[async_trait::async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, error::ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(error::ReservationError::InvalidTime);
        };

        let start: NaiveDateTime =
            NaiveDateTime::from_timestamp_opt(rsvp.start.unwrap().seconds.into(), 0).unwrap();
        let end = NaiveDateTime::from_timestamp_opt(rsvp.end.unwrap().seconds.into(), 0).unwrap();
        if start <= end {
            return Err(error::ReservationError::InvalidTime);
        };
        // generate a insert sql for the reservation
        let sql = "INSERT INTO reservation(user_iid, resource_id, timespan, note ,status) VALUES($1,$2,$3,$4,$5) RETURNING id";
        // Execute the sql
        let id = sqlx::query!(
            sql,
            rsvp.user_id,
            rsvp.resource_id,
            rsvp.timespan,
            rsvp.note,
            rsvp.rtype
        );
        todo!()
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
