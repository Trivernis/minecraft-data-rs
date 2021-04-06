use std::io;
use thiserror::Error;

pub type DataResult<T> = Result<T, DataError>;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("IO Error: {0}")]
    IOError(#[from] io::Error),

    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Object {0} not found")]
    NotFoundError(String),

    #[error("Invalid encoding of file {0}")]
    InvalidEncodingError(String),
}
