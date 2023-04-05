use std::ops::Deref;

use crate::{
    types::{reservation_date::NaiveDateRange, reservation_status::RsvpStatus},
    *,
};
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::{types::PgRange, PgRow},
    FromRow, Row,
};

use super::{get_timespan, validate_range};

impl Reservation {
    pub fn new_pending<'a>(
        user_id: impl Into<String>,
        resource_id: impl Into<String>,
        start: &str,
        end: &str,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            resource_id: resource_id.into(),
            status: ReservationStatus::Pending as i32,
            user_id: user_id.into(),
            start: Some(convert_to_timestamp(
                &start.parse::<DateTime<Utc>>().unwrap(),
            )),
            end: Some(convert_to_timestamp(&end.parse::<DateTime<Utc>>().unwrap())),
            note: note.into(),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(self.user_id.clone()));
        }

        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(self.resource_id.clone()));
        }

        validate_range(self.start.as_ref(), self.end.as_ref())
    }

    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }

    pub fn get_timespan_string(&self) -> String {
        let start = convert_timestamp_to_naiveDt(self.start.as_ref().unwrap());
        let end = convert_timestamp_to_naiveDt(self.end.as_ref().unwrap());

        format!("[{}, {})", start, end)
    }
}

impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // `PgRange<DateTime<Utc>>` 与 Postgres `TSTZRANGE` 对应的类型
        let time_range: PgRange<DateTime<Utc>> = row.get("timespan");
        let n_d_r: NaiveDateRange = time_range.into();

        let status: RsvpStatus = row.get("status");

        let id: i64 = row.get("id");

        Ok(Self {
            id,
            resource_id: row.get("resource_id"),
            user_id: row.get("user_id"),
            status: ReservationStatus::from(status) as i32,
            start: Some(convert_naiveDt_to_timestamp(&n_d_r.start.unwrap())),
            end: Some(convert_naiveDt_to_timestamp(&n_d_r.end.unwrap())),
            note: row.get("note"),
        })
    }
}

impl Borrow for ValueInt64 {}
