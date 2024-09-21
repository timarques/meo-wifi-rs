mod error;
mod legacy;

pub (super) use legacy::Legacy;
pub (super) use error::Error;

pub (super) trait Trait: Clone {
    fn is_logged(&self) -> bool;
    fn login(&self) -> Result<(), error::Error>;
    fn logout(&self) -> Result<(), error::Error>;
}