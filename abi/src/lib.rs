mod error;
mod pb;

use std::fmt::Display;

pub use error::*;
pub use pb::*;

use chrono::prelude::*;
use prost_types::Timestamp;

#[warn(non_snake_case)]
pub fn convert_str_to_timestamp(dt: &str) -> Timestamp {
    let date_time = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").unwrap();
    let ts = date_time.timestamp_millis();

    Timestamp {
        seconds: (ts / 1000) as i64,
        nanos: ((ts % 1000) * 1_000_000) as i32,
    }
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

impl Reservation {
    pub fn new_pending<'a>(
        // id: impl Into<String>,
        user_id: impl Into<String>,
        resource_id: impl Into<String>,
        // status: impl Into<i32>,
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
            return Err(Error::UserIdEmpty);
        }

        if self.resource_id.is_empty() {
            return Err(Error::ResourceIdEmpty);
        }

        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime);
        } else {
            if self.start.clone().unwrap().seconds >= self.end.clone().unwrap().seconds {
                return Err(Error::InvalidTime);
            }
        }

        Ok(())
    }

    pub fn get_timespan(&self) -> std::ops::Range<NaiveDateTime> {
        let start = self.start.clone().unwrap();
        let end = self.end.clone().unwrap();

        let start =
            NaiveDateTime::from_timestamp_opt(start.seconds, start.nanos.try_into().unwrap())
                .unwrap();
        let end =
            NaiveDateTime::from_timestamp_opt(end.seconds, end.nanos.try_into().unwrap()).unwrap();
        std::ops::Range { start, end }
    }
}
