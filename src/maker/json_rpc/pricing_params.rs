use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingParams {
    pub pairs: Vec<Pair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_expiry: Option<String>,
}

impl PricingParams {
    pub fn new(pairs: Vec<Pair>) -> Self {
        Self {
            pairs,
            min_expiry: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    base_token: String,
    quote_token: String,
}

impl Pair {
    pub fn new(base_token: String, quote_token: String) -> Self {
        Self {
            base_token,
            quote_token,
        }
    }
}
