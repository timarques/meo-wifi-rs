use crate::connections::Error as ConnectionError;
use crate::session::Error as SessionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Connection(ConnectionError),
    Session(SessionError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Connection(err) => &err.to_string(),
            Self::Session(err) => &err.to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}

impl From<ConnectionError> for Error {
    fn from(e: ConnectionError) -> Self {
        Self::Connection(e)
    }
}

impl From<SessionError> for Error {
    fn from(e: SessionError) -> Self {
        Self::Session(e)
    }
}