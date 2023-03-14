use std::fmt::Display;

use crate::ReservationStatus;

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
