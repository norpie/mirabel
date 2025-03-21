use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    // Generic error type
    #[error("Generic error, with message: {0}")]
    Generic(String),

    // # Custom
    // Search engine error types
    #[error("No available search engine")]
    NoAvailableEngine,

    // 400.. HTTP error types
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("Resource not found: {0}")]
    NotFound(String),

    // 500.. HTTP error types
    #[error("Internal server error")]
    InternalServer,

    // Library error types
    #[error("An actix error occurred: {0}")]
    ActixWeb(#[from] actix_web::Error),
    #[error("An argon2 error occurred: {0}")]
    Argon2(#[from] argon2::password_hash::Error),
    #[error("A chrono error occurred: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("A serde_json error occurred: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("A reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("A reqwest_streams error occurred: {0}")]
    StreamBody(#[from] reqwest_streams::error::StreamBodyError),
    #[error("A surrealdb error occurred: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("An eyre report occurred: {0}")]
    EyreReport(#[from] eyre::Report),
    #[error("A jsonwebtoken error occurred: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error("A fantoccini error occurred: {0}")]
    FantocciniSession(#[from] Box<fantoccini::error::NewSessionError>),
    #[error("A fantoccini error occurred: {0}")]
    FantocciniCmd(#[from] Box<fantoccini::error::CmdError>),

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
