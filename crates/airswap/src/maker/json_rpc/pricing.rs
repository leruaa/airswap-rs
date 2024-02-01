use serde::Deserialize;
use serde_json::Number;

use super::Level;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub ask: Vec<Level>,
    pub bid: Vec<Level>,
    pub base_token: String,
    pub quote_token: String,
    pub minimum: Number,
}

impl Pricing {
    pub fn get_best_bid(&self) -> Option<&Level> {
        self.bid.last()
    }
}
