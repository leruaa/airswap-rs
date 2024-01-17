use serde::Deserialize;

use super::{ResponseError, ResponseResult};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Result(ResponseResult),
    Error(ResponseError),
}
