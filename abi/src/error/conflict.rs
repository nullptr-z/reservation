use std::{convert::Infallible, str::FromStr};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum ReservationConflictInfo {
    Parsed(ReservationConflict),
    Unparsed(String),
}

#[derive(Debug, Clone)]
pub struct ReservationConflict {
    a: ReservationWindow,
    b: ReservationWindow,
}

#[derive(Debug, Clone)]
pub struct ReservationWindow {
    rid: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl FromStr for ReservationConflictInfo {
    type Err = Infallible; // 不会出错

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(conflict) = s.parse() {
            Ok(ReservationConflictInfo::Parsed(conflict))
        } else {
            Ok(ReservationConflictInfo::Unparsed(s.to_string()))
        }
    }
}

impl FromStr for ReservationConflict {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use regular expression to parse the string

        todo!()
    }
}
