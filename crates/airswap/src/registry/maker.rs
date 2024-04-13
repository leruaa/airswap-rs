use std::{fmt::Display, sync::Arc};

use alloy::primitives::Address;

use crate::{registry::KNOWN_MAKERS, MakerWithSupportedTokens};

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

impl Display for Maker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maker_name = KNOWN_MAKERS
            .get(&self.address)
            .cloned()
            .unwrap_or(format!("{}", self.address));

        write!(f, "{maker_name}")
    }
}
