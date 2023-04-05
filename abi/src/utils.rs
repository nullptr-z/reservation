use chrono::prelude::*;
use prost_types::Timestamp;

pub fn convert_str_to_timestamp(dt: &str) -> Timestamp {
    dt.parse::<Timestamp>().unwrap()

    // let date_time = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").unwrap();
    // let utc_datetime: DateTime<Utc> = Utc.from_utc_datetime(&date_time); // convert to UTC timezone
    // let dt = utc_datetime.timestamp_millis();

    // Timestamp {
    //     seconds: (dt / 1000) as i64,
    //     nanos: utc_datetime.timestamp_subsec_nanos() as _,
    // }
}

pub fn convert_naiveDt_to_timestamp(time: &NaiveDateTime) -> Timestamp {
    Timestamp {
        seconds: time.timestamp(),
        nanos: time.timestamp_nanos() as _,
    }
}

pub fn convert_timestamp_to_str(timestamp: &Timestamp) -> String {
    let dt = Local.timestamp_opt(timestamp.seconds, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn convert_timestamp_to_date_time(timestamp: &Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(convert_timestamp_to_naiveDt(timestamp), Utc)
}

pub fn convert_timestamp_to_naiveDt(timestamp: &Timestamp) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_millis(timestamp.seconds * 1000).unwrap()
}

pub fn convert_str_to_naiveDt(str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S").unwrap()
}

pub fn convert_to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as _,
    }
}

pub fn str_to_datetime_utc(s: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S%.f%z").unwrap()
}
