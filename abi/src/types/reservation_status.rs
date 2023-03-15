use std::fmt::Display;

use crate::ReservationStatus;

// 对应数据中 TYPE，用于获取数据库 status Type 到本地 status Struct的映射
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Unknown,
    Pending,
    Confirmed,
    Blocked,
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

impl From<RsvpStatus> for ReservationStatus {
    fn from(status: RsvpStatus) -> Self {
        match status {
            RsvpStatus::Unknown => ReservationStatus::Unknown,
            RsvpStatus::Pending => ReservationStatus::Pending,
            RsvpStatus::Confirmed => ReservationStatus::Confirmed,
            RsvpStatus::Blocked => ReservationStatus::Blocked,
        }
    }
}
