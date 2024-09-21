mod continuous;
mod oneshot;
mod error;

pub (super) use continuous::Continuous;
pub (super) use oneshot::Oneshot;
pub (super) use error::Error;

pub (super) trait Trait {
    fn execute(&self) -> Result<(), error::Error>;
}