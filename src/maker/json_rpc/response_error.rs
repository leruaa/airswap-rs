use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use super::ErrorPayload;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub jsonrpc: String,
    pub error: ErrorPayload,
    pub id: Value,
}

#[derive(Error, Debug, Clone)]
pub enum ResponseDecodeError {
    #[error("{0}")]
    Remote(ErrorPayload),
    #[error("Wrong variant")]
    WrongVariant,
}
