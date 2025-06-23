use miette::Diagnostic;
use scraper::error::SelectorErrorKind;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::dto::session::event::SessionEvent;

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
    #[error("Resource not found")]
    NotFound,
    #[error("Resource not while updating: {0}")]
    NotFoundRecentUpdate(String),

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
    SurrealDB(Box<surrealdb::Error>),
    #[error("An eyre report occurred: {0}")]
    EyreReport(#[from] eyre::Report),
    #[error("A jsonwebtoken error occurred: {0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error("A fantoccini error occurred: {0}")]
    FantocciniSession(Box<fantoccini::error::NewSessionError>),
    #[error("A fantoccini error occurred: {0}")]
    FantocciniCmd(Box<fantoccini::error::CmdError>),
    #[error("A deadpool error occurred: {0}")]
    Deadpool(#[from] deadpool::unmanaged::PoolError),
    #[error("A regex error occurred: {0}")]
    Regex(#[from] regex::Error),
    #[error("A lopdf error occurred: {0}")]
    Lopdf(#[from] lopdf::Error),
    #[error("A pdf_extract error occurred: {0}")]
    PdfExtract(#[from] pdf_extract::Error),
    #[error("A pdf_extract output error occurred: {0}")]
    PdfExtractOutput(#[from] pdf_extract::OutputError),
    #[error("A scraper error occurred: {0}")]
    Scraper(String), // TODO: Find a way to keep more information

    // `std`-error types
    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
    #[error("An error occurred while parsing a string: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("An error occurred while parsing an integer: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Environment variable error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Channel error")]
    Channel(Box<SendError<SessionEvent>>),
    #[error("Double subscription error")]
    DoubleSubscription,
    #[error("A socket was closed unexpectedly")]
    SocketClosed,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl<'a> From<SelectorErrorKind<'a>> for Error {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        Error::Scraper(format!("{:?}", error))
    }
}

impl From<fantoccini::error::NewSessionError> for Error {
    fn from(e: fantoccini::error::NewSessionError) -> Self {
        Error::FantocciniSession(Box::new(e))
    }
}

impl From<fantoccini::error::CmdError> for Error {
    fn from(e: fantoccini::error::CmdError) -> Self {
        Error::FantocciniCmd(Box::new(e))
    }
}

impl From<SendError<SessionEvent>> for Error {
    fn from(e: SendError<SessionEvent>) -> Self {
        Error::Channel(Box::new(e))
    }
}

impl From<surrealdb::Error> for Error {
    fn from(e: surrealdb::Error) -> Self {
        Error::SurrealDB(Box::new(e))
    }
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
