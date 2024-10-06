use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::Payload;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Payload,
    pub id: String,
}

impl From<Payload> for Request {
    fn from(params: Payload) -> Self {
        let id = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .to_string();

        let method = match &params {
            Payload::Protocols => String::from("getProtocols"),
            Payload::SignerSideOrder(_) => String::from("getSignerSideOrderERC20"),
            Payload::SenderSideOrder(_) => String::from("getSenderSideOrderERC20"),
            Payload::Pricing(_) => String::from("getPricingERC20"),
            Payload::AllPricing => String::from("getAllPricingERC20"),
        };

        Request {
            jsonrpc: String::from("2.0"),
            method,
            params,
            id,
        }
    }
}
