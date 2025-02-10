use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("An error occurred, which uses the generic error type {0}")]
    Generic(String),

    #[error("A serde_json error occurred: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("An IO error occurred: {0}")]
    IO(#[from] std::io::Error),
}
