mod conflict;

pub use conflict::*;
use sqlx::postgres::PgDatabaseError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),

    #[error("Invalid reservation id: {0}")]
    InvalidReservationId(i64),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(i64),

    #[error("Invalid page cursor: {0}")]
    InvalidCursor(i64),
    #[error("Invalid status: {0}")]
    InvalidStatus(i32),

    #[error("Invalid start: {0} or end: {1} time for the reservation")]
    InvalidTime(String, String),

    #[error("DataBase errr")]
    DbError(sqlx::Error),

    #[error("No reservation found by the given condition` 没有找到给定的条件预定")]
    NotFound,

    #[error("Conflict revation")]
    ConflictReservation(ReservationConflictInfo),

    #[error("unknown error")]
    Unknown,

    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to read configuration file")]
    ConfigParseError,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DbError(_), Self::DbError(_)) => true,
            (Self::InvalidTime(_, _), Self::InvalidTime(_, _)) => true,
            (Self::ConflictReservation(v1), Self::ConflictReservation(v2)) => v1 == v2,
            (Self::NotFound, Self::NotFound) => true,
            (Self::InvalidUserId(v1), Self::InvalidUserId(v2)) => v1 == v2,
            (Self::InvalidResourceId(v1), Self::InvalidResourceId(v2)) => v1 == v2,
            (Self::Unknown, Self::Unknown) => true,
            _ => false,
        }
    }

    // fn ne(&self, other: &Self) -> bool {
    //     !self.eq(other)
    // }
}

// @0 sqlx::Error的错误在这里做额外处理
impl From<sqlx::Error> for Error {
    fn from(s_err: sqlx::Error) -> Self {
        match s_err {
            sqlx::Error::Database(db_err) => {
                let err: &PgDatabaseError = db_err.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    // @2 处理 23P01 错误
                    ("23P01", Some("rsvp"), Some("reservation")) => {
                        // 对 23P01 格式化处理
                        // parse调用将 string for ReservationConflictInfo
                        // 最终呈现 ReservationWindow
                        Error::ConflictReservation(err.detail().unwrap().parse().unwrap())
                    }
                    _ => {
                        println!("{}: {:?}", err.code(), err);
                        Error::DbError(sqlx::Error::Database(db_err))
                    }
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(s_err),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::DbError(_) | Error::ConfigReadError | Error::ConfigParseError => {
                tonic::Status::internal(e.to_string())
            }
            Error::InvalidUserId(_)
            | Error::InvalidStatus(_)
            | Error::InvalidResourceId(_)
            | Error::InvalidReservationId(_)
            | Error::InvalidTime(_, _)
            | Error::InvalidPageSize(_)
            | Error::InvalidCursor(_) => tonic::Status::invalid_argument(e.to_string()),
            Error::NotFound => {
                tonic::Status::not_found("No reservation found by the given condition")
            }
            Error::ConflictReservation(info) => {
                tonic::Status::failed_precondition(format!("Conflict reservation: {:?}", info))
            }
            Error::Unknown => tonic::Status::unknown("Unknown eroor"),
        }
    }
}
