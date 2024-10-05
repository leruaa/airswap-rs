use serde::Deserialize;

use super::{ErrorPayload, OrderPayload, PricingPayload};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ResultPayload {
    SignerSideOrder(Box<OrderPayload>),
    Pricing(Box<PricingPayload>),
    Error(Box<ErrorPayload>),
}
