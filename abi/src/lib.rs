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
