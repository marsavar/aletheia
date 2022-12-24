//! Error variants.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
}
