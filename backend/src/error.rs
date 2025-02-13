use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    // Generic error type
    #[error("Generic error, with message: {0}")]
    Generic(String),

    // 400.. HTTP error types
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Resource not found: {0}")]
    NotFound(String),

    // 500.. HTTP error types
    #[error("Internal server error")]
    InternalServer,

    // Library error types
    #[error("An actix error occurred: {0}")]
    ActixWeb(#[from] actix_web::Error),
    #[error("A chrono error occurred: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("A serde_json error occurred: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("A reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("A reqwest_streams error occurred: {0}")]
    StreamBody(#[from] reqwest_streams::error::StreamBodyError),

    // `std`-error types
    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
    #[error("An error occurred while parsing an integer: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("An error occurred while parsing a float: {0}")]
    Var(#[from] std::env::VarError),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}
