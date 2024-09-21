#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    CredentialsMismatch,
    InvalidIp,
    InvalidEmail,
    InvalidPassword,
    NetworkUnreachable,
    AlreadyLoggedIn,
    Custom(String)
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CredentialsMismatch => write!(f, "Credentials do not match."),
            Error::InvalidIp => write!(f, "Invalid IP address."),
            Error::InvalidEmail => write!(f, "Invalid email address."),
            Error::InvalidPassword => write!(f, "Invalid password."),
            Error::NetworkUnreachable => write!(f, "Network is unreachable."),
            Error::AlreadyLoggedIn => write!(f, "User is already logged in."),
            Error::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::Custom(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Custom(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Custom(e.to_string())
    }
}