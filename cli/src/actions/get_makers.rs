use airswap::RegistryClient;
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use num_traits::ToPrimitive;
use std::{fmt::Display, sync::Arc};

use crate::cli::Config;

use super::action::Action;

pub struct GetMakersAction {
    config: Config,
}

impl GetMakersAction {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl Action for GetMakersAction {
    async fn execute(&self) -> Result<()> {
        let provider = ProviderBuilder::new().on_http(self.config.rpc.parse()?)?;
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry = RegistryClient::new(provider, chain_id, self.config.registry_version);

        let makers = registry
            .get_makers_with_supported_tokens()
            .await?
            .into_iter()
            .map(Maker::from)
            .collect::<Vec<_>>();

        let table = makers
            .table()
            .border(Border::builder().build())
            .separator(Separator::builder().build());

        print_stdout(table)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Table)]
pub struct Maker {
    pub address: String,
    pub url: String,
    pub status: MakerStatus,
}

#[derive(Debug, Clone, Copy)]
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
