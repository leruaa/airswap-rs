use std::sync::Arc;

use alloy_primitives::Address;

use crate::MakerWithSupportedTokens;

#[derive(Debug, Clone)]
pub struct Maker {
    pub address: Address,
    pub url: String,
}

impl Maker {
    pub fn new(address: Address, url: String) -> Self {
        Self { address, url }
    }
}

impl From<Arc<MakerWithSupportedTokens>> for Maker {
    fn from(value: Arc<MakerWithSupportedTokens>) -> Self {
        value.maker.clone()
    }
}
