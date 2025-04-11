//! Error types for the Zenobuf CLI

use std::fmt;

/// Error type for the Zenobuf CLI
#[derive(Debug)]
pub enum Error {
    /// Zenoh error
    Zenoh(zenoh::Error),
    /// JSON error
    Json(serde_json::Error),
    /// Key expression error
    KeyExpr(String),
    /// Other error
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Zenoh(e) => write!(f, "Zenoh error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::KeyExpr(e) => write!(f, "Key expression error: {}", e),
            Error::Other(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<zenoh::Error> for Error {
    fn from(e: zenoh::Error) -> Self {
        Error::Zenoh(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Other(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Other(e.to_string())
    }
}

/// Result type for the Zenobuf CLI
pub type Result<T> = std::result::Result<T, Error>;
