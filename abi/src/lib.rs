mod pb;

pub use pb::*;

use chrono::DateTime;
use prost_types::Timestamp;

pub fn convert_str_to_Timestamp(dt: &str) -> Timestamp {
    let dt = DateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").unwrap();
    let ts = dt.timestamp_millis();

    Timestamp {
        seconds: (ts / 1000) as i64,
        nanos: ((ts % 1000) * 1_000_000) as i32,
    }
}
