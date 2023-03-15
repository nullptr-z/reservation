use chrono::prelude::*;
use prost_types::Timestamp;

pub fn convert_str_to_timestamp(dt: &str) -> Timestamp {
    let date_time = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").unwrap();
    let utc_datetime: DateTime<Utc> = Utc.from_utc_datetime(&date_time); // convert to UTC timezone
    let dt = utc_datetime.timestamp_millis();

    Timestamp {
        seconds: (dt / 1000) as i64,
        nanos: ((dt % 1000) * 1_000_000) as i32,
    }
}

pub fn convert_naiveDt_to_timestamp(time: NaiveDateTime) -> Timestamp {
    Timestamp {
        seconds: time.timestamp(),
        nanos: time.timestamp_nanos() as i32,
    }
}

pub fn convert_timestamp_to_str(timestamp: Timestamp) -> String {
    let dt = Local.timestamp_opt(timestamp.seconds, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn convert_timestamp_to_naiveDt(timestamp: Timestamp) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(timestamp.seconds, timestamp.nanos.try_into().unwrap())
        .unwrap()
}

pub fn convert_str_to_naiveDt(str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S").unwrap()
}
