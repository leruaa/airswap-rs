use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

impl Display for ErrorPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            Some(data) => write!(f, "{} ({}): {}", self.message, self.code, data),
            None => write!(f, "{} ({})", self.message, self.code),
        }
    }
}
