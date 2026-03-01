use airswap::{Config as AirswapConfig, RegistryClient};
use alloy::providers::{Provider, ProviderBuilder};
use alloy_erc20::{BasicTokenStore, TokenId, TokenStore};
use anyhow::{anyhow, Result};
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use num_traits::ToPrimitive;
use std::{collections::HashSet, fmt::Display, sync::Arc};

use crate::cli::Config;

use super::action::Action;

pub struct GetMakersAction {
    symbols: Vec<String>,
    config: Config,
}

impl GetMakersAction {
    pub fn new(symbols: Vec<String>, config: Config) -> Self {
        Self { symbols, config }
    }
}

#[async_trait::async_trait]
impl Action for GetMakersAction {
    async fn execute(&self) -> Result<()> {
        let provider = ProviderBuilder::new().connect_http(self.config.rpc.parse()?);
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let config = AirswapConfig::new(chain_id, self.config.protocol_version);
        let registry = RegistryClient::new(provider, config);
        let mut makers = HashSet::new();
        let mut store = BasicTokenStore::new();

        store.insert_known_tokens(chain_id);

        for symbol in &self.symbols {
            let token = store
                .get(chain_id, TokenId::Symbol(symbol.clone()))
                .ok_or(anyhow!("The token {} can't be found", symbol))?;

            let makers_for_token = registry
                .get_makers_with_supported_tokens(token.address)
                .await?;

            makers_for_token.into_iter().for_each(|m| {
                makers.insert(m);
            });
        }

        let makers: Vec<Maker> = makers.into_iter().map(Maker::from).collect();

        let table = makers
            .table()
            .border(Border::builder().build())
            .separator(Separator::builder().build());

        print_stdout(table)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Table, Eq, Hash, PartialEq)]
pub struct Maker {
    pub address: String,
    pub url: String,
    pub status: MakerStatus,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MakerStatus {
    Active,
    Inactive,
}

impl Display for MakerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MakerStatus::Active => write!(f, "Active"),
            MakerStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

impl From<usize> for MakerStatus {
    fn from(value: usize) -> Self {
        match value {
            0 => MakerStatus::Inactive,
            _ => MakerStatus::Active,
        }
    }
}

impl From<airswap::MakerWithSupportedTokens> for Maker {
    fn from(value: airswap::MakerWithSupportedTokens) -> Self {
        Self {
            address: format!("{:?}", value.maker.address),
            url: value.maker.url,
            status: value.supported_tokens.len().into(),
        }
    }
}
