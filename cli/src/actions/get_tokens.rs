use airswap::{Config as AirswapConfig, RegistryClient};
use alloy::providers::{Provider, ProviderBuilder};
use alloy_erc20::{BasicTokenStore, Erc20ProviderExt};
use anyhow::Result;
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use num_traits::ToPrimitive;

use crate::cli::Config;

use super::Action;

pub struct GetTokensAction {
    config: Config,
    maker_address: String,
}

impl GetTokensAction {
    pub fn new(config: Config, maker_address: String) -> Self {
        Self {
            config,
            maker_address,
        }
    }
}

#[async_trait::async_trait]
impl Action for GetTokensAction {
    async fn execute(&self) -> Result<()> {
        let provider = ProviderBuilder::new().on_http(self.config.rpc.parse()?);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let config = AirswapConfig::new(chain_id, self.config.protocol_version);
        let registry_client = RegistryClient::new(provider.clone(), config);

        let mut token_store = BasicTokenStore::new();

        let supported_tokens = registry_client
            .get_tokens(self.maker_address.parse().unwrap())
            .await?;
        let mut tokens = vec![];

        for address in supported_tokens {
            let token = provider.get_token(address, &mut token_store).await?;

            tokens.push(Token::from(token));
        }

        let table = tokens
            .table()
            .border(Border::builder().build())
            .separator(Separator::builder().build());

        print_stdout(table)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Table)]
pub struct Token {
    pub address: String,
    pub symbol: String,
}

impl From<&alloy_erc20::Token> for Token {
    fn from(value: &alloy_erc20::Token) -> Self {
        Self {
            address: format!("{:?}", value.address),
            symbol: value.symbol.clone(),
        }
    }
}
