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
where
    C: Connections,
    S: Session,
{
    pub fn new(connections: &'a C, session: &'a S, target: &'a str) -> Self {
        Self {
            connections,
            session,
            target,
        }
    }

    fn setup_connection(&self) -> Result<(), Error> {
        if let Some(main_connection) = self.connections.active() {
            if main_connection != self.target {
                log::warn("Disconnecting non-target main connection");
                self.connections.disconnect(&main_connection)?;
                log::info("Main connection disconnected");
            }
        }

        if self.connections.is_connected(self.target) {
            log::info("Target connection already active");
        } else {
            log::warn("Target connection not active, connecting");
            self.connections.connect(self.target)?;
            log::info("Target connection activated");
        }

        Ok(())
    }

    fn setup_session(&self) -> Result<(), Error> {
        if self.session.is_logged() {
            log::info("Session already active");
        } else {
            log::warn("Session not active, logging in");
            self.session.login()?;
            log::info("Session logged in successfully");
        }

        Ok(())
    }
}

impl<C, S> Trait for Oneshot<'_, C, S>
where
    C: Connections,
    S: Session,
{
    fn execute(&self) -> Result<(), Error> {
        self.setup_connection().and_then(|_| self.setup_session())
    }
}
