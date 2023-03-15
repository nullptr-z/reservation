use std::ops::Bound;

use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::postgres::types::PgRange;

pub struct NaiveDateRange {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

impl From<PgRange<DateTime<Utc>>> for NaiveDateRange {
    fn from(pg_r: PgRange<DateTime<Utc>>) -> Self {
        let ret_time = |date: Bound<DateTime<Utc>>| match date {
            std::ops::Bound::Included(v) => Some(v.naive_utc()),
            std::ops::Bound::Excluded(v) => Some(v.naive_utc()),
            std::ops::Bound::Unbounded => None,
        };

        let start = ret_time(pg_r.start);
        let end = ret_time(pg_r.end);
        Self { start, end }
    }
}

// 有过痛苦、针扎，我还是无法装作不知道，现在我决定离开了；如果他能就是你命中注定的那个人，那么我也祝你幸福

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
