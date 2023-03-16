use std::ops::Bound;

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sqlx::postgres::types::PgRange;

use crate::{convert_timestamp_to_date_time, convert_timestamp_to_str, Error};

mod reservation;
mod reservation_date;
mod reservation_query;
mod reservation_status;

// pub use reservation::*;
// pub use reservation_date::*;
// pub use reservation_status::*;

pub fn validate_range(
    start: Option<&Timestamp>,
    end: Option<&Timestamp>,
) -> Result<(), Error<'static>> {
    if start.is_none() || end.is_none() {
        return Err(Error::InvalidTime(
            convert_timestamp_to_str(start.unwrap()),
            convert_timestamp_to_str(end.unwrap()),
        ));
    }

    let start = start.unwrap();
    let end = end.unwrap();

    if start.seconds >= end.seconds {
        return Err(Error::InvalidTime(
            convert_timestamp_to_str(start),
            convert_timestamp_to_str(end),
        ));
    }

    Ok(())
}

pub fn get_timespan(start: Option<&Timestamp>, end: Option<&Timestamp>) -> PgRange<DateTime<Utc>> {
    PgRange {
        start: Bound::Included(convert_timestamp_to_date_time(start.as_ref().unwrap())),
        end: Bound::Included(convert_timestamp_to_date_time(end.as_ref().unwrap())),
    }
}
