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

pub fn validate_range(start: Option<&Timestamp>, end: Option<&Timestamp>) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use prost_types::Timestamp;

    use crate::{convert_timestamp_to_date_time, types::validate_range};

    use super::get_timespan;

    #[test]
    fn validate_range_should_allow_correct_range() {
        let start = Timestamp {
            seconds: 1,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 2,
            nanos: 0,
        };

        assert!(validate_range(Some(&start), Some(&end)).is_ok());
    }

    #[test]
    fn validate_range_should_reject_invalid_range() {
        let start = Timestamp {
            seconds: 2,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 1,
            nanos: 0,
        };

        assert!(validate_range(Some(&start), Some(&end)).is_err());
    }

    #[test]
    fn get_timespan_should_work_for_valid_start_end() {
        let start = Timestamp {
            seconds: 1,
            nanos: 0,
        };
        let end = Timestamp {
            seconds: 2,
            nanos: 0,
        };

        let range = get_timespan(Some(&start), Some(&end));

        assert_eq!(
            range.start,
            Bound::Included(convert_timestamp_to_date_time(&start))
        );
        assert_eq!(
            range.end,
            Bound::Included(convert_timestamp_to_date_time(&end))
        );
    }
}
