//! Error variants.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Missing query parameter: {0}")]
    MissingQueryParameter(&'static str),
    #[error("API error: {0}")]
    ApiError(String),
}
