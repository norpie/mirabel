use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("An error occurred, which uses the generic error type {0}")]
    Generic(String),

    #[error("An error occurred: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("A serde_json error occurred: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("A reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("A reqwest_streams error occurred: {0}")]
    StreamBody(#[from] reqwest_streams::error::StreamBodyError),

    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
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
