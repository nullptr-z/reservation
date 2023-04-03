use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;

use crate::{convert_str_to_timestamp, ReservationQuery, ReservationStatus, Validate};

use super::{get_timespan, validate_range};

impl ReservationQuery {
    pub fn new(
        uid: impl Into<String>,
        rid: impl Into<String>,
        status: ReservationStatus,
        start: impl Into<String>,
        end: impl Into<String>,
        desc: bool,
    ) -> Self {
        Self {
            user_id: uid.into(),
            resource_id: rid.into(),
            status: status as i32,
            start: Some(convert_str_to_timestamp(&start.into())),
            end: Some(convert_str_to_timestamp(&end.into())),
            desc,
        }
    }

    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}

impl Validate for ReservationQuery {
    fn validate(&self) -> Result<(), crate::Error> {
        validate_range(self.start.as_ref(), self.end.as_ref())
    }
}
