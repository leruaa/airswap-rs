use airswap::{MakerClient, MakerWithSupportedTokens, RegistryClient};
use alloy::{
    network::Ethereum,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
};
use alloy_primitives::{utils::parse_units, Address};
use anyhow::Result;
use cli_table::{
    format::{Border, Separator},
    print_stdout, Table,
};
use erc20::{
    clients::{CachableTokenClient, TokenClient},
    stores::{BasicTokenStore, TokenStore},
    TokenId,
};
use futures::future::join_all;
use itertools::Itertools;
use num_traits::ToPrimitive;
use std::sync::Arc;

use super::Action;
use crate::cli::{BuyCommand, Config, SellCommand};

pub struct QuoteAction {
    config: Config,
    from_symbol: String,
    to_symbol: String,
    amount: Side,
    maker: Option<Address>,
}

impl QuoteAction {
    pub fn buy(config: Config, command: BuyCommand) -> Self {
        Self {
            config,
            from_symbol: command.from_symbol,
            to_symbol: command.to_symbol,
            amount: Side::Buy(command.to_amount),
            maker: command.maker,
        }
    }

    pub fn sell(config: Config, command: SellCommand) -> Self {
        Self {
            config,
            from_symbol: command.from_symbol,
            to_symbol: command.to_symbol,
            amount: Side::Sell(command.from_amount),
            maker: command.maker,
        }
    }
}

#[async_trait::async_trait]
impl Action for QuoteAction {
    async fn execute(&self) -> Result<()> {
        let rpc_client = RpcClient::builder().reqwest_http(self.config.rpc.parse()?);
        let provider = ProviderBuilder::<_, Ethereum>::new().on_client(rpc_client);
        let provider = Arc::new(provider);
        let chain_id = provider.get_chain_id().await?.to_u64().unwrap();
        let registry_client =
            RegistryClient::new(provider.clone(), chain_id, self.config.registry_version);

        let mut store = BasicTokenStore::new();

        store.insert_known_tokens(chain_id as u8);

        let token_client =
            CachableTokenClient::new(TokenClient::new(provider), chain_id as u8, store);

        let from_address = "0x5615dEB798BB3E4dFa0139dFa1b3D433Cc23b72f"
            .parse()
            .unwrap();
        let from_token = token_client
            .retrieve_token(TokenId::Symbol(self.from_symbol.clone()))
            .await?;
        let to_token = token_client
            .retrieve_token(TokenId::Symbol(self.to_symbol.clone()))
            .await?;

        let makers = registry_client
            .get_makers_with_supported_tokens()
            .await?
            .into_iter()
            .filter(|m: &MakerWithSupportedTokens| !m.supported_tokens.is_empty())
            .filter(|m| {
                m.supported_tokens.contains(&from_token.address)
                    && m.supported_tokens.contains(&to_token.address)
            })
            .filter(|m| {
                self.maker
                    .map(|address| address == *m.address())
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        let tasks = makers.into_iter().map(|m| {
            let amount = self.amount.clone();
            let from_token = from_token.clone();
            let to_token = to_token.clone();

            tokio::spawn(async move {
                let maker_client = MakerClient::new(chain_id, m.clone());
                let quote = match amount {
                    Side::Buy(amount) => {
                        let amount = parse_units(&amount.to_string(), from_token.decimals).unwrap();
                        maker_client
                            .get_buy_quote(
                                from_address,
                                from_token.address,
                                to_token.address,
                                amount.into(),
                            )
                            .await
                    }
                    Side::Sell(amount) => {
                        let amount = parse_units(&amount.to_string(), from_token.decimals).unwrap();
                        maker_client
                            .get_sell_quote(
                                from_address,
                                from_token.address,
                                to_token.address,
                                amount.into(),
                            )
                            .await
                    }
                };

                match quote {
                    Ok(quote) => Quote::new(
                        m.maker.url.clone(),
                        format!("{}", to_token.get_balance(quote.signer_amount)),
                    ),
                    Err(err) => Quote::new(m.maker.url.clone(), format!("{:#}", err)),
                }
            })
        });

        let (quotes, _) = join_all(tasks)
            .await
            .into_iter()
            .partition_result::<Vec<_>, Vec<_>, _, _>();

        let table = quotes
            .table()
            .border(Border::builder().build())
            .separator(Separator::builder().build());

        print_stdout(table)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Table)]
pub struct Quote {
    pub maker: String,
    pub result: String,
}

impl Quote {
    pub fn new(maker: String, result: String) -> Self {
        Self { maker, result }
    }
}

#[derive(Clone, Debug)]
enum Side {
    Buy(f64),
    Sell(f64),
}
