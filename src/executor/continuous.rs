use super::{Oneshot, Error, Trait};
use crate::log;
use crate::session::Trait as Session;
use crate::connections::Trait as Connections;
use crate::connections::Error as ConnectionError;
use std::time::Duration;
use std::thread;

pub struct Continuous<'a, C: Connections, S: Session> {
    connections: &'a C,
    session: &'a S,
    target: &'a str,
    original: Option<String>,
    check_interval: Duration,
    oneshot: Oneshot<'a, C, S>
}

impl<'a, C: Connections, S: Session> Continuous<'a, C, S> {

    fn has_internet_connection() -> bool {
        const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);
        const TEST_HOSTS: [&str; 3] = [
            "1.1.1.1:80", 
            "8.8.8.8:53", 
            "google.com:80"
        ];
        TEST_HOSTS
            .iter()
            .any(|&host| {
                std::net::TcpStream::connect_timeout(&host.parse().unwrap(), TIMEOUT)
                    .is_ok()
            })
    }

    pub fn new(connections: &'a C, session: &'a S, target: &'a str) -> Self {
        const DEFAULT_INTERVAL: Duration = Duration::from_secs(60);
        let original = connections.active();
        let oneshot = Oneshot::new(
            connections, 
            session, 
            target
        );
        Self {
            connections,
            session,
            target,
            original,
            check_interval: DEFAULT_INTERVAL,
            oneshot
        }
    }

    fn restore_original_connection(&self) -> Result<(), Error> {
        match &self.original {
            Some(connection) => self.connections.connect(&connection).map_err(|e| e.into()),
            None => self.connections.disconnect(&self.target).map_err(|e| e.into()),
        }
    }

    fn reconnect_and_login(&self) -> Result<(), Error> {
        log::info("No internet connection, attempting to reconnect");
        let _ = self.session.logout().map_err(log::error);
        self.connections.reconnect()?;
        log::info("Connection reconnected");
        let _ = self.session.login().map_err(log::error);
        log::info("Login attempt completed");
        Ok(())
    }

    fn ensure_connectivity(&self) -> Result<(), Error> {
        self.oneshot.execute()?;
        if Self::has_internet_connection() {
            return Ok(());
        }
        self.reconnect_and_login()?;
        if !Self::has_internet_connection() {
            return Err(Error::Connection(ConnectionError::NoInternet));
        }
        Ok(())
    }
}

impl<C: Connections, S: Session> super::Trait for Continuous<'_, C, S> {
    fn execute(&self) -> Result<(), Error> {
        loop {
            if let Err(error) = self.ensure_connectivity() {
                self.restore_original_connection()?;
                log::info("Original connection restored");
                return Err(error.into());
            }
            thread::sleep(self.check_interval);
        }
    }
}