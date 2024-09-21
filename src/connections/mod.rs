mod nmcli;
mod error;

pub (super) use error::Error;
pub (super) use nmcli::Nmcli;

pub (super) trait Trait: Clone {

    fn active(&self) -> Option<String>;
    fn connect(&self, connection: &str) -> Result<(), error::Error>;
    fn disconnect(&self, connection: &str) -> Result<(), error::Error>;
    fn is_connected(&self, name: &str) -> bool;
    fn reconnect(&self) -> Result<(), error::Error> {
        if let Some(active) = self.active() {
            self.disconnect(&active)?;
            self.connect(&active)?;
            Ok(())
        } else {
            Err(error::Error::Unavailable)
        }
    }

}