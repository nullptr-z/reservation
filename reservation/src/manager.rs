use crate::{ReservationId, ReservationManager, Rsvp};
use abi::{Error, ReservationStatus};
use sqlx::{types::Uuid, Row};

impl ReservationManager {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

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
        // if current status is pending,change it to confirmed, otherwise do nothing
        let rsvp:abi::Reservation=sqlx::query_as(
            "UPDATE rsvp.reservation SET status = 'confirmed' WHERE id = $1::Uuid AND status = 'pending' RETURNING *",
        ).bind(id).fetch_one(&self.pool).await?;

        Ok(rsvp)
    }

    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, Error> {
        // update the note of  reservation by id
        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservation SET note = $1 WHERE id = $2 RETURNING *")
                .bind(note)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn delete(&self, id: ReservationId) -> Result<(), Error> {
        // delete the reservation by id
        sqlx::query("DELETE FROM rsvp.reservation WHERE id = $1::Uuid RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        // get the reservation by id
        let rsvp: abi::Reservation =
            sqlx::query_as("SELECT * FROM rsvp.reservation WHERE id = $1::Uuid")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use abi::{
        convert_str_to_naiveDt, Reservation, ReservationConflict, ReservationConflictInfo,
        ReservationStatus, ReservationWindow,
    };

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_validate_window() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let rsvp = create_alice_reservation(&manager).await;
        assert_ne!(rsvp.id, "");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_should_reject() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let _rsvp = create_alice_reservation(&manager).await;
        let rsvp_err = create_try_reservation(&manager).await.unwrap_err();
        println!("{:?}", rsvp_err);

        let info = ReservationConflictInfo::Parsed(ReservationConflict {
            new: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: convert_str_to_naiveDt("2023-03-11 12:00:00"),
                end: convert_str_to_naiveDt("2023-03-13 12:00:00"),
            },
            old: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: convert_str_to_naiveDt("2023-03-11 12:00:00"),
                end: convert_str_to_naiveDt("2023-03-12 12:00:00"),
            },
        });

        assert_eq!(rsvp_err, Error::ConflictReservation(info));
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_should_work() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp = create_alice_reservation(&manager).await;

        let new_rsvp = manager.update_status(rsvp.id).await.unwrap();
        assert_eq!(new_rsvp.status, ReservationStatus::Confirmed as i32);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_not_pending_should_do_nothing() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp = create_alice_reservation(&manager).await;

        let new_rsvp = manager.update_status(rsvp.id).await.unwrap();
        // change status again should do nothing`再次改变状态应该什么都不做
        let ret = manager.update_status(new_rsvp.id).await.unwrap_err();
        assert_eq!(ret, Error::NotFound);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_note_should_work() {}

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_not_confirmed_should_do_nothing() {}

    //#region 用于构造 Reservation 工具函数
    async fn create_alice_reservation(manager: &ReservationManager) -> Reservation {
        create_reservation(
            manager,
            "change test",
            "ocean-view-room-713",
            "2023-03-11 12:00:00",
            "2023-03-12 12:00:00",
            "i'm today evening to check in",
        )
        .await
    }

    // 重复的订单，日期会冲突
    async fn create_try_reservation(manager: &ReservationManager) -> Result<Reservation, Error> {
        let rsvp = create_conflict_reservation(
            manager,
            "zzz id",
            "ocean-view-room-713",
            "2023-03-11 12:00:00",
            "2023-03-13 12:00:00",
            "用于冲突测试",
        )
        .await?;

        Ok(rsvp)
    }

    async fn create_reservation(
        manager: &ReservationManager,
        uid: &str,
        rid: &str,
        start: &str,
        end: &str,
        note: &str,
    ) -> Reservation {
        let rsvp = abi::Reservation::new_pending(uid, rid, start, end, note);

        let rsvp = manager.reserve(rsvp).await.unwrap();

        rsvp
    }

    async fn create_conflict_reservation<'a>(
        manager: &'a ReservationManager,
        uid: &'a str,
        rid: &'a str,
        start: &'a str,
        end: &'a str,
        note: &'a str,
    ) -> Result<Reservation, Error<'a>> {
        let rsvp = abi::Reservation::new_pending(uid, rid, start, end, note);

        let rsvp = manager.reserve(rsvp).await?;

        Ok(rsvp)
    }
    //#endRegion
}
