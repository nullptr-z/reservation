use std::fmt::Display;

use chrono::NaiveDateTime;
use sqlx::{
    postgres::{types::PgRange, PgRow},
    types::Uuid,
    FromRow, Row,
};

use crate::{convert_naiveDt_to_timestamp, Reservation, ReservationStatus};

// 对应数据中 TYPE，用于获取数据库 status Type 到本地 status Struct的映射
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Unknown,
    Pending,
    Confirmed,
    Blocked,
}

impl Display for ReservationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReservationStatus::Pending => f.write_str("pending"),
            ReservationStatus::Blocked => f.write_str("blocked"),
            ReservationStatus::Confirmed => f.write_str("confirmed"),
            ReservationStatus::Unknown => f.write_str("unknown"),
        }
    }
}

impl From<RsvpStatus> for ReservationStatus {
    fn from(status: RsvpStatus) -> Self {
        match status {
            RsvpStatus::Unknown => ReservationStatus::Unknown,
            RsvpStatus::Pending => ReservationStatus::Pending,
            RsvpStatus::Confirmed => ReservationStatus::Confirmed,
            RsvpStatus::Blocked => ReservationStatus::Blocked,
        }
    }
}

impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let time_range: PgRange<NaiveDateTime> = row.get("teimespan");
        let n_d_r: NaiveDateRange = time_range.into();

        let status: RsvpStatus = row.get("status");

        let id: Uuid = row.get("id");

        Ok(Self {
            id: id.to_string(),
            resource_id: row.get("resource_id"),
            user_id: row.get("user_id"),
            status: ReservationStatus::from(status) as i32,
            start: Some(convert_naiveDt_to_timestamp(n_d_r.start.unwrap())),
            end: Some(convert_naiveDt_to_timestamp(n_d_r.end.unwrap())),
            note: row.get("note"),
        })
    }
}

// 有过痛苦、针扎，我还是无法装作不知道，现在我决定离开了；如果他能就是你命中注定的那个人，那么我也祝你幸福
struct NaiveDateRange {
    start: Option<NaiveDateTime>,
    end: Option<NaiveDateTime>,
}

impl From<PgRange<NaiveDateTime>> for NaiveDateRange {
    fn from(pg_r: PgRange<NaiveDateTime>) -> Self {
        let ret_time = |date| match date {
            std::ops::Bound::Included(v) => Some(v),
            std::ops::Bound::Excluded(v) => Some(v),
            std::ops::Bound::Unbounded => None,
        };

        let start = ret_time(pg_r.start);
        let end = ret_time(pg_r.end);
        Self { start, end }
    }
}
