use std::sync::Arc;

use airswap::{json_rpc::Pair, MakerClient, RegistryClient};
use alloy::{
    network::Ethereum,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
};
use anyhow::Result;
use erc20::{
    clients::{CachableTokenClient, TokenClient},
    stores::BasicTokenStore,
    TokenId,
};
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
        let rpc_client = RpcClient::builder().reqwest_http(self.config.rpc.parse()?);
        let provider = ProviderBuilder::<_, Ethereum>::new().on_client(rpc_client);
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);

        let maker = registry_client
            .get_maker_with_supported_tokens(self.maker_address.parse()?)
            .await?;

        let maker_client = MakerClient::new(chain_id, maker);

        let token_client = CachableTokenClient::new(
            TokenClient::new(provider),
            chain_id as u8,
            BasicTokenStore::new(),
        );

        let form_token = token_client
            .retrieve_token(TokenId::Symbol(self.from_symbol.clone()))
            .await?;
        let to_token = token_client
            .retrieve_token(TokenId::Symbol(self.to_symbol.clone()))
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
