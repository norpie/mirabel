use std::{error::Error as AStdError, sync::PoisonError};

use diesel::PgConnection;
use miette::Diagnostic;
use scraper::error::SelectorErrorKind;
use std::sync::MutexGuard;
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
    #[error("Resource not found")]
    NotFound,
    #[error("Resource not while updating: {0}")]
    NotFoundRecentUpdate(String),
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    #[error("Conflict: {0}")]
    Conflict(String),

    // 500.. HTTP error types
    #[error("Internal server error")]
    InternalServer,

    // # DB
    // Diesel (async)
    #[error("A diesel error occurred: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("A diesel pool error occurred: {0}")]
    DieselPool(#[from] deadpool_diesel::PoolError),
    #[error("A diesel interact error occurred: {0}")]
    DieselInteract(#[from] deadpool_diesel::InteractError),
    #[error("A diesel connection error occurred: {0}")]
    DieselConnection(#[from] diesel::ConnectionError),
    #[error("A diesel pool build error occurred: {0}")]
    DieselPoolBuild(#[from] deadpool_diesel::postgres::BuildError),

    // Poison
    #[error("A poisoned lock error occurred: {0}")]
    PoisonedLock(String),

    // Library error types
    #[error("An actix error occurred: {0}")]
    ActixWeb(#[from] actix_web::Error),
    #[error("An argon2 error occurred: {0}")]
    Argon2(String),
    #[error("A chrono error occurred: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("A serde_json error occurred: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("A reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("A reqwest_streams error occurred: {0}")]
    StreamBody(#[from] reqwest_streams::error::StreamBodyError),
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
    #[error("A Tera error occurred: {0}")]
    Tera(#[from] tera::Error),

    // `std`-error types
    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
    #[error("An error occurred while parsing a string: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("An error occurred while parsing an integer: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Environment variable error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Double subscription error")]
    DoubleSubscription,
    #[error("A socket was closed unexpectedly")]
    SocketClosed,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<Box<dyn AStdError + std::marker::Send + std::marker::Sync>> for Error {
    fn from(error: Box<dyn AStdError + std::marker::Send + std::marker::Sync>) -> Self {
        Error::Generic(format!("{error:?}"))
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        Error::Argon2(format!("{error:?}"))
    }
}

impl<'a> From<PoisonError<MutexGuard<'a, Option<PgConnection>>>> for Error {
    fn from(error: PoisonError<MutexGuard<'a, Option<PgConnection>>>) -> Self {
        Error::PoisonedLock(format!("{error:?}"))
    }
}

impl<'a> From<SelectorErrorKind<'a>> for Error {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        Error::Scraper(format!("{error:?}"))
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
