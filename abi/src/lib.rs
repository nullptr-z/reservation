mod pb;

use std::fmt::Display;

pub use pb::*;

use chrono::prelude::*;
use prost_types::Timestamp;

#[warn(non_snake_case)]
pub fn convert_str_to_timestamp(dt: &str) -> Timestamp {
    let datetime = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").unwrap();
    let ts = datetime.timestamp_millis();

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
