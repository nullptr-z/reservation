use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("user id is empty")]
    UserIdEmpty,
    #[error("resource id is empty")]
    ResourceIdEmpty,
    #[error("DataBase errr")]
    DbError(#[from] sqlx::Error),
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,
    #[error("unknown error")]
    Unknown,
}
