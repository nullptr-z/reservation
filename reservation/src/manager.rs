use crate::{ReservationId, ReservationManager, Rsvp};
use abi::{Error, ReservationStatus};
use sqlx::{types::Uuid, Row};

#[async_trait::async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, Error> {
        rsvp.validate().unwrap();

        let timespan = rsvp.get_timespan();

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
            "2023-03-11 17:20:35",
            "2023-03-12 17:20:35",
            "i'm today evening to check in",
        );

        let rsvp = manager.reserve(rsvp).await.expect("create reserve error");
        assert_ne!(rsvp.id, "");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_should_reject() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp1 = abi::Reservation::new_pending(
            "zz id",
            "ocean-view-room-713",
            "2023-03-11 17:20:35",
            "2023-03-12 17:20:35",
            "i'm today evening to check in",
        );
        let rsvp2 = abi::Reservation::new_pending(
            "zzz id",
            "ocean-view-room-713",
            "2023-03-11 17:20:35",
            "2023-03-13 17:20:35",
            "i'm today evening to check in",
        );

        let rsvp = manager.reserve(rsvp1).await.unwrap();
        let rsvp_err = manager.reserve(rsvp2).await.unwrap();

        println!("{:?}", rsvp_err);
    }
}
