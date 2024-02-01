use alloy_primitives::Address;

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
}
