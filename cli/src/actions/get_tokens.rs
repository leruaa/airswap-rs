use std::sync::Arc;

use airswap::RegistryClient;
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use erc20::{Erc20Provider, TokenId};
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
        let provider = ProviderBuilder::new().on_http(self.config.rpc.parse()?)?;
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);
        let erc20_provider = Erc20Provider::new(provider, chain_id as u8);

        let supported_tokens = registry_client
            .get_tokens(self.maker_address.parse().unwrap())
            .await?;
        let mut tokens = vec![];

        for address in supported_tokens {
            let token = erc20_provider
                .retrieve_token(TokenId::Address(address))
                .await?;

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

impl From<Arc<erc20::Token>> for Token {
    fn from(value: Arc<erc20::Token>) -> Self {
        Self {
            address: format!("{:?}", value.address),
            symbol: value.symbol.clone(),
        }
    }
}
