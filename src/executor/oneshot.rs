use crate::log;
use crate::session::Trait as Session;
use crate::connections::Trait as Connections;
use super::{Error, Trait};

pub struct Oneshot<'a, C: Connections, S: Session> {
    connections: &'a C,
    session: &'a S,
    target: &'a str,
}

impl<'a, C, S> Oneshot<'a, C, S>
where C: Connections, S: Session
{

    pub fn new(connections: &'a C, session: &'a S, target: &'a str) -> Self {
        Self {
            connections,
            session,
            target,
        }
    }

    fn setup_connection(&self) -> Result<(), Error> {
        // Disconnect the primary connection before attempting to connect to the target network.
        // This ensures that nmcli selects the correct network device for the connection.
        if let Some(main_connection) = self.connections.active() {
            if main_connection != self.target {
                log::warn("connection not main");
                self.connections.disconnect(&main_connection)?;
                log::info("main connection disconnected");
            }
        }

        if self.connections.is_connected(self.target) {
            log::info("connection already active");
        } else {
            log::warn("connection not active");
            self.connections.connect(&self.target)?;
            log::info("connection active");
        }

        Ok(())
    }

    fn setup_session(&self) -> Result<(), Error> {
        if self.session.is_logged() {
            log::info("session active");
        } else {
            log::warn("session not active");
            self.session.login()?;
            log::info("session logged in");
        }
        Ok(())
    }

}

impl<C, S> Trait for Oneshot<'_, C, S>
where C: Connections, S: Session
{
    fn execute(&self) -> Result<(), Error> {
        self.setup_connection()?;
        self.setup_session()
    }
}
