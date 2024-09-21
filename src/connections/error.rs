#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AlreadyActive,
    Unavailable,
    NoInternet,
    Custom(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::AlreadyActive => "Connection is already active",
            Self::Unavailable => "Connection is unavailable",
            Self::NoInternet => "No internet connection",
            Self::Custom(msg) => msg
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Custom(e.to_string())
    }
}