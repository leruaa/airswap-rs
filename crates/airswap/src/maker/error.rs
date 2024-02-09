use reqwest::StatusCode;
use thiserror::Error;

use crate::json_rpc::ResponseDecodeError;

#[derive(Error, Debug)]
pub enum MakerError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Conversion(#[from] ResponseDecodeError),
    #[error("Empty response")]
    EmptyResponse,
    #[error("Server error: {0}")]
    ServerError(StatusCode),
    #[error("Rate limit met")]
    RateLimitMet,
    #[error("The pair is not supported")]
    PairNotSupported,
}
