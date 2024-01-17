use serde::Deserialize;
use serde_json::Value;

use super::ResultPayload;

#[derive(Debug, Deserialize)]
pub struct ResponseResult {
    pub jsonrpc: String,
    pub result: ResultPayload,
    pub id: Value,
}
