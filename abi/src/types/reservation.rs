use crate::{
    types::{reservation_date::NaiveDateRange, reservation_status::RsvpStatus},
    *,
};
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::{types::PgRange, PgRow},
    types::Uuid,
    FromRow, Row,
};

impl Reservation {
    pub fn new_pending<'a>(
        user_id: impl Into<String>,
        resource_id: impl Into<String>,
        start: impl Into<&'a str>,
        end: impl Into<&'a str>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: "".into(),
            resource_id: resource_id.into(),
            status: ReservationStatus::Pending as i32,
            user_id: user_id.into(),
            start: Some(convert_str_to_timestamp(start.into())),
            end: Some(convert_str_to_timestamp(end.into())),
            note: note.into(),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(&self.user_id));
        }

        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(&self.resource_id));
        }

        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime(
                convert_timestamp_to_str(self.start.clone().unwrap()),
                convert_timestamp_to_str(self.end.clone().unwrap()),
            ));
        } else {
            let start = self.start.clone().unwrap();
            let end = self.end.clone().unwrap();
            if start.seconds >= end.seconds {
                return Err(Error::InvalidTime(
                    convert_timestamp_to_str(start),
                    convert_timestamp_to_str(end),
                ));
            }
        }

        Ok(())
    }

    pub fn get_timespan(&self) -> String {
        let start = convert_timestamp_to_naiveDt(self.start.clone().unwrap());
        let end = convert_timestamp_to_naiveDt(self.end.clone().unwrap());

        format!("[{}, {})", start, end)
    }
}

impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        // `PgRange<DateTime<Utc>>` 与 Postgres `TSTZRANGE` 对应的类型
        let time_range: PgRange<DateTime<Utc>> = row.get("timespan");
        println!("time_range->{:?}", time_range);
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
