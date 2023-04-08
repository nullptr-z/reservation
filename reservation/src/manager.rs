use crate::*;
use sqlx::{postgres::PgPoolOptions, Either, Row};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tracing::{info, warn};

impl ReservationManager {
    // 接受一个sqlx::PgPool，将它包裹起来
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    // Get Pool<Postgres> through DbConfig configuration
    // 通过DbConfig配置得到Pool<Postgres>
    pub async fn from_config(config: &DbConfig) -> Result<Self, Error> {
        let url = config.url();
        let pool = PgPoolOptions::default()
            .max_connections(config.max_connections)
            .connect(&url)
            .await?;
        // ::connect(&url).await?;
        Ok(Self::new(pool))
    }
}

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, Error> {
        rsvp.validate().unwrap();

        let timespan = rsvp.get_timespan();

        let status = ReservationStatus::from_i32(rsvp.status).unwrap_or(ReservationStatus::Pending);

        let id:i64 = sqlx::query(
            "INSERT INTO rsvp.reservation (user_id, resource_id, timespan, note ,status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id;
        Ok(rsvp)
    }

    async fn update_status(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        // if current status is pending,change it to confirmed, otherwise do nothing
        id.validate().unwrap();
        let rsvp:abi::Reservation=sqlx::query_as(
            "UPDATE rsvp.reservation SET status = 'confirmed' WHERE id = $1 AND status = 'pending' RETURNING *",
        ).bind(id).fetch_one(&self.pool).await?;

        Ok(rsvp)
    }

    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, Error> {
        // update the note of  reservation by id
        id.validate().unwrap();
        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservation SET note = $1 WHERE id = $2 RETURNING *")
                .bind(note)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn delete(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        // delete the reservation by id
        id.validate().unwrap();
        let rsvp: Reservation =
            sqlx::query_as("DELETE FROM rsvp.reservation WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, Error> {
        // get the reservation by id
        id.validate().unwrap();
        let rsvp: abi::Reservation = sqlx::query_as("SELECT * FROM rsvp.reservation WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(rsvp)
    }

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, Error>> {
        // let time_range = query.get_timespan();
        let start = query
            .start
            .map(|time| convert_timestamp_to_date_time(&time));
        let end = query.end.map(|time| convert_timestamp_to_date_time(&time));

        let status = ReservationStatus::from_i32(query.status).unwrap();
        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut rsvps = sqlx::query_as(
                "SELECT * FROM rsvp.query($1, $2, $3, $4, $5::rsvp.reservation_status, $6)",
            )
            .bind(query.user_id)
            .bind(query.resource_id)
            .bind(start)
            .bind(end)
            .bind(status.to_string())
            .bind(query.desc)
            .fetch_many(&pool);
            while let Some(ret) = rsvps.next().await {
                match ret {
                    Ok(Either::Left(r)) => {
                        info!("Query result: {:?}", r);
                    }
                    Ok(Either::Right(r)) => {
                        if tx.send(Ok(r)).await.is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        warn!("Query result error: {:?}", err);
                        if tx.send(Err(err.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        rx
    }

    async fn filter(
        &self,
        mut filter: ReservationFilter,
    ) -> Result<(FilterPager, Vec<abi::Reservation>), Error> {
        filter.normalize()?;

        let sql = filter.to_sql()?;

        let rsvps: Vec<Reservation> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;
        let mut rsvps = rsvps.into_iter().collect();

        let pager = filter.get_pager(&mut rsvps);

        Ok((pager, rsvps.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use abi::{
        convert_str_to_naiveDt, Reservation, ReservationConflict, ReservationConflictInfo,
        ReservationQueryBuilder, ReservationStatus, ReservationWindow,
    };

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_validate_window() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let rsvp = create_alice_reservation(&manager).await;
        assert_ne!(rsvp.id, 0);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_should_reject() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let _rsvp = create_alice_reservation(&manager).await;
        let rsvp_err = create_try_reservation(&manager).await.unwrap_err();

        let info = ReservationConflictInfo::Parsed(ReservationConflict {
            new: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: convert_str_to_naiveDt("2023-03-11 12:00:00"),
                end: convert_str_to_naiveDt("2023-03-13 12:00:00"),
            },
            old: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: convert_str_to_naiveDt("2023-03-11 12:00:00"),
                end: convert_str_to_naiveDt("2023-03-13 12:00:00"),
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

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn query_reservation_should_work() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let rsvp = create_alice_reservation(&manager).await;
        let query = ReservationQueryBuilder::default()
            .user_id(rsvp.user_id.clone())
            .resource_id(rsvp.resource_id.clone())
            .start(rsvp.start.clone().unwrap())
            .end(rsvp.end.clone().unwrap())
            .status(ReservationStatus::Pending.into())
            .build()
            .unwrap();

        let mut rx = manager.query(query).await;
        assert_eq!(rx.recv().await, Some(Ok(rsvp)))
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn filter_reservation_should_work() {
        let manager = ReservationManager::new(migrated_pool.clone());

        let rsvp = create_alice_reservation(&manager).await;

        let filter = ReservationFilterBuilder::default()
            .user_id(rsvp.user_id.clone())
            // .cursor(value)
            .status(ReservationStatus::Pending)
            .build()
            .unwrap();

        let (pager, rsvps) = manager.filter(filter).await.unwrap();
        assert!(rsvps.len() > 0);
        assert_eq!(pager.prev, None);
        assert_eq!(pager.next, None);
        assert_eq!(rsvps[0], rsvp);
    }

    //#region 用于构造 Reservation 工具函数
    async fn create_alice_reservation(manager: &ReservationManager) -> Reservation {
        create_reservation(
            manager,
            "test uid",
            "ocean-view-room-713",
            "2023-03-11 12:00:00Z",
            "2023-03-13 12:00:00Z",
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
            "2023-03-11 12:00:00Z",
            "2023-03-13 12:00:00Z",
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
    ) -> Result<Reservation, Error> {
        let rsvp = abi::Reservation::new_pending(uid, rid, start, end, note);

        let rsvp = manager.reserve(rsvp).await?;

        Ok(rsvp)
    }
    //#endRegion
}
