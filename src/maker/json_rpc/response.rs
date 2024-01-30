use serde::Deserialize;
use serde_json::Value;

use super::{ResponseError, ResponseResult};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Result(ResponseResult),
    Error(ResponseError),
    Unknown(Value),
}
