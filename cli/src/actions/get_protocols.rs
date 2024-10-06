use airswap::{MakerClient, RegistryClient};
use alloy::providers::{Provider, ProviderBuilder};
use alloy_erc20::{BasicTokenStore, TokenStore};
use anyhow::Result;
use num_traits::ToPrimitive;

use crate::cli::Config;

use super::Action;

pub struct GetProtocolsAction {
    config: Config,
    maker_address: String,
}

impl GetProtocolsAction {
    pub fn new(config: Config, maker_address: String) -> Self {
        Self {
            config,
            maker_address,
        }
    }
}

#[async_trait::async_trait]
impl Action for GetProtocolsAction {
    async fn execute(&self) -> Result<()> {
        let provider = ProviderBuilder::new().on_http(self.config.rpc.parse()?);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);

        let mut token_store = BasicTokenStore::new();

        token_store.insert_known_tokens(chain_id);

        let maker = registry_client
            .get_maker_with_supported_tokens(self.maker_address.parse()?)
            .await?;

        let maker_client = MakerClient::new(chain_id, maker);

        let protocols = maker_client.get_protocols().await?;

        println!("{protocols:?}");

        Ok(())
    }
}
