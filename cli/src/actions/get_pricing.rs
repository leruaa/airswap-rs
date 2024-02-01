use std::sync::Arc;

use airswap::{json_rpc::Pair, MakerClient, RegistryClient};
use alloy_providers::provider::{Provider, TempProvider};
use alloy_rpc_client::RpcClient;
use anyhow::Result;
use erc20::{TokenId, TokenStore};
use num_traits::ToPrimitive;

use crate::cli::Config;

use super::Action;

pub struct GetPricingAction {
    config: Config,
    maker_address: String,
    from_symbol: String,
    to_symbol: String,
}

impl GetPricingAction {
    pub fn new(
        config: Config,
        maker_address: String,
        from_symbol: String,
        to_symbol: String,
    ) -> Self {
        Self {
            config,
            maker_address,
            from_symbol,
            to_symbol,
        }
    }
}

#[async_trait::async_trait]
impl Action for GetPricingAction {
    async fn execute(&self) -> Result<()> {
        let provider = Provider::new_with_client(
            RpcClient::builder()
                .reqwest_http(self.config.rpc.parse().unwrap())
                .boxed(),
        );
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);

        let maker = registry_client
            .get_maker(self.maker_address.parse()?)
            .await?;

        let maker_client = MakerClient::new(chain_id, maker.url);

        let token_store = TokenStore::new(chain_id, provider);

        let form_token = token_store
            .get(TokenId::Symbol(self.from_symbol.clone()))
            .await?;
        let to_token = token_store
            .get(TokenId::Symbol(self.to_symbol.clone()))
            .await?;

        let pricing = maker_client
            .get_pricing(vec![Pair::new(
                format!("{:?}", form_token.address),
                format!("{:?}", to_token.address),
            )])
            .await?;

        println!("{pricing:#?}");

        Ok(())
    }
}
