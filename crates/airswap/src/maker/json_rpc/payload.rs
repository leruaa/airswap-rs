use serde::{Deserialize, Serialize};

use super::{PricingParams, SenderSideOrderParams, SignerSideOrderParams};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    SignerSideOrder(SignerSideOrderParams),
    SenderSideOrder(SenderSideOrderParams),
    Pricing(PricingParams),
    AllPricing(Vec<String>),
}
