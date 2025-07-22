use std::fmt;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Hash(String),
    PasswordVerification,
    Serialization(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Generic error: {msg}"),
            Error::Hash(msg) => write!(f, "Hash error: {msg}"),
            Error::PasswordVerification => write!(f, "Password verification failed"),
            Error::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            Error::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            Error::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            Error::Forbidden(msg) => write!(f, "Forbidden: {msg}"),
            Error::NotFound => write!(f, "Not found"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
