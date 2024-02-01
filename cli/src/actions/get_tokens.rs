use std::sync::Arc;

use airswap::RegistryClient;
use alloy_providers::provider::{Provider, TempProvider};
use alloy_rpc_client::RpcClient;
use anyhow::Result;
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use erc20::{TokenId, TokenStore};
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
        let provider = Provider::new_with_client(
            RpcClient::builder()
                .reqwest_http(self.config.rpc.parse().unwrap())
                .boxed(),
        );
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);
        let token_store = TokenStore::new(chain_id, provider);

        let supported_tokens = registry_client
            .get_tokens(self.maker_address.parse().unwrap())
            .await?;
        let mut tokens = vec![];

        for address in supported_tokens {
            let token = token_store.get(TokenId::Address(address)).await?;

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
