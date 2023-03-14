mod conflict;

pub use conflict::*;
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Invalid user id: {0}")]
    InvalidUserId(&'a str),
    #[error("Invalid resource id: {0}")]
    InvalidResourceId(&'a str),
    #[error("DataBase errr")]
    DbError(sqlx::Error),
    #[error("Conflict revation")]
    ConflictReservation(ReservationConflictInfo),
    #[error("Invalid start: {0} or end: {1} time for the reservation")]
    InvalidTime(String, String),
    #[error("unknown error")]
    Unknown,
}

// @0 sqlx::Error的错误在这里做额外处理
impl From<sqlx::Error> for Error<'_> {
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
                    _ => Error::DbError(sqlx::Error::Database(db_err)),
                }
            }
            _ => Error::DbError(s_err),
        }
    }
}
