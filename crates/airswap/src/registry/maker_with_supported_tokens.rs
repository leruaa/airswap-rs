use std::fmt::Display;

use alloy::primitives::Address;

use crate::Maker;

#[derive(Debug, Clone)]
pub struct MakerWithSupportedTokens {
    pub maker: Maker,
    pub supported_tokens: Vec<Address>,
}

impl MakerWithSupportedTokens {
    pub fn new(maker: Maker, supported_tokens: Vec<Address>) -> Self {
        Self {
            maker,
            supported_tokens,
        }
    }

    pub fn address(&self) -> &Address {
        &self.maker.address
    }

    pub fn url(&self) -> String {
        self.maker.url.clone()
    }

    pub fn can_handle(&self, addresses: &[Address]) -> bool {
        addresses.iter().all(|a| self.supported_tokens.contains(a))
    }
}

impl Display for MakerWithSupportedTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.maker.fmt(f)
    }
}
