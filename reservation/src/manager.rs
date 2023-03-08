use crate::{error, ReservationId, ReservationManager, Rsvp};

#[async_trait::async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, error::ReservationError> {
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
