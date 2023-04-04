mod config;
mod error;
mod pb;
mod types;
mod utils;

pub use config::*;
pub use error::*;
pub use pb::*;
pub use utils::*;

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

// validate an normalize the data structure
pub trait Normalizer: Validate {
    // caller should call normalize to make sure the data structure is ready to use
    // 调用者应调用 normalize 以确保数据结构已准备好使用
    fn normalize(&mut self) -> Result<(), Error> {
        self.validate();
        self.do_nomalize();
        Ok(())
    }

    // user shall implement do_normalize() to normalize the data structure
    fn do_nomalize(&mut self);
}

pub trait ToSql {
    fn to_sql(&self) -> Result<String, Error>;
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
