use serde::Deserialize;

use super::{ErrorPayload, OrderPayload, PricingPayload, ProtocolsPayload};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ResultPayload {
    Protocols(Box<ProtocolsPayload>),
    SignerSideOrder(Box<OrderPayload>),
    Pricing(Box<PricingPayload>),
    Error(Box<ErrorPayload>),
}
