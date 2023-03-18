mod error;
mod pb;
mod types;
mod utils;

pub use error::*;
pub use pb::*;
pub use utils::*;
// pub use types::*;

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

pub type ReservationId = i64;
pub type UserId = String;
pub type ResourceId = String;

impl Validate for ReservationId {
    fn validate(&self) -> Result<(), Error> {
        if *self <= 0 {
            Err(Error::InvalidReservationId(*self))
        } else {
            Ok(())
        }
    }
}
