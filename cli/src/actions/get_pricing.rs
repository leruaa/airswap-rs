use airswap::{json_rpc::Pair, MakerClient, RegistryClient};
use alloy::providers::{Provider, ProviderBuilder};
use alloy_erc20::{BasicTokenStore, Erc20ProviderExt, TokenId, TokenStore};
use anyhow::Result;
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

        let form_token = provider
            .get_token(
                self.from_symbol.parse::<TokenId>().unwrap(),
                &mut token_store,
            )
            .await?;

        let from_token = format!("{:?}", form_token.address);

        let to_token = provider
            .get_token(self.to_symbol.parse::<TokenId>().unwrap(), &mut token_store)
            .await?;

        let to_token = format!("{:?}", to_token.address);

        let pricing = maker_client
            .get_pricing(vec![Pair::new(from_token, to_token)])
            .await?;

        println!("{pricing:#?}");

        Ok(())
    }
}
