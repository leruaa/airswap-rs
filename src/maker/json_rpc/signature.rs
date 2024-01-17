use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signatory: String,
    pub validator: String,
    pub version: String,
    pub r: String,
    pub s: String,
    pub v: String,
}
