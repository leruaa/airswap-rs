use std::sync::Arc;

use alloy::primitives::Address;
use alloy::sol_types::{SolCall, SolEvent};
use alloy::{
    network::{Network, TransactionBuilder},
    providers::{Provider, RootProvider},
    rpc::types::eth::Filter,
    sol,
    transports::{Transport, TransportError},
};
use async_trait::async_trait;
use futures::{future::try_join_all, TryFutureExt};
use thiserror::Error;

use crate::{Maker, MakerWithSupportedTokens};

use super::{config::RegistryConfig, RegistryVersion};

sol!(LegacyRegistryContract, "abi/registry.json");
sol!(RegistryV4Contract, "abi/registry_v4.json");

#[async_trait]
pub trait RegistryContract: Send + Sync {
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError>;
    async fn get_makers(&self) -> Result<Vec<Maker>, RegistryError>;
    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError>;
}

async fn call<C, T, N>(
    provider: &Arc<RootProvider<T, N>>,
    call: C,
    to: Address,
) -> Result<C::Return, RegistryError>
where
    C: SolCall + Send + Sync,
    T: Transport + Clone,
    N: Network,
{
    let tx = N::TransactionRequest::default()
        .with_input(call.abi_encode())
        .with_to(to);

    let result = provider.call(&tx).await?;
    let decoded = C::abi_decode_returns(&result, true)?;

    Ok(decoded)
}

async fn get_makers_events<E, T, N>(
    provider: &Arc<RootProvider<T, N>>,
    config: &RegistryConfig,
) -> Result<Vec<E>, RegistryError>
where
    E: SolEvent,
    T: Transport + Clone,
    N: Network,
{
    let filter = Filter::new()
        .from_block(config.from_block)
        .address(config.address)
        .event(E::SIGNATURE);

    let set_url_events = provider.get_logs(&filter).await?;

    set_url_events
        .into_iter()
        .map(|log| E::decode_log_data(log.data(), true).map_err(RegistryError::from))
        .collect::<Result<Vec<_>, _>>()
}

fn normalized_maker(account: Address, mut url: String) -> Maker {
    if url.contains("wintermute") {
        //continue;
    }

    if url.starts_with("wss://") {
        //continue;
    }

    //if !url.starts_with("http") {
    //url = format!("https://{}", url);
    //}

    url = url.replace('\"', "");

    Maker::new(account, url)
}

pub struct RegistryClient {
    inner: Box<dyn RegistryContract>,
}

impl RegistryClient {
    pub fn new<T, N>(
        provider: Arc<RootProvider<T, N>>,
        chain_id: u64,
        version: RegistryVersion,
    ) -> Self
    where
        N: Network,
        T: Transport + Clone,
    {
        match version {
            RegistryVersion::Legacy => Self {
                inner: Box::new(LegacyRegistry::new(provider, chain_id, version)),
            },
            RegistryVersion::V4 => Self {
                inner: Box::new(RegistryV4::new(provider, chain_id, version)),
            },
        }
    }

    pub async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        self.inner.get_maker(address).await
    }

    pub async fn get_maker_with_supported_tokens(
        &self,
        address: Address,
    ) -> Result<MakerWithSupportedTokens, RegistryError> {
        let maker = self.inner.get_maker(address).await?;
        let supported_tokens = self.get_tokens(maker.address).await?;

        Ok(MakerWithSupportedTokens::new(maker, supported_tokens))
    }

    pub async fn get_makers(&self) -> Result<Vec<Maker>, RegistryError> {
        self.inner.get_makers().await
    }

    pub async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        self.inner.get_tokens(maker_address).await
    }

    pub async fn get_makers_with_supported_tokens(
        &self,
    ) -> Result<Vec<MakerWithSupportedTokens>, RegistryError> {
        let futures = self.inner.get_makers().await?.into_iter().map(|m| {
            self.get_tokens(m.address)
                .map_ok(|supported_tokens| MakerWithSupportedTokens::new(m, supported_tokens))
        });

        let makers_with_supported_tokens = try_join_all(futures).await?;

        Ok(makers_with_supported_tokens)
    }
}

pub struct LegacyRegistry<N, T> {
    provider: Arc<RootProvider<N, T>>,
    config: RegistryConfig,
}

impl<N, T> LegacyRegistry<N, T> {
    pub fn new(provider: Arc<RootProvider<N, T>>, chain_id: u64, version: RegistryVersion) -> Self {
        let config = RegistryConfig::new(chain_id, version);

        Self { provider, config }
    }
}

#[async_trait]
impl<T, N> RegistryContract for LegacyRegistry<T, N>
where
    T: Transport + Clone + Send + Sync,
    N: Network + Send + Sync,
{
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        let url = call(
            &self.provider,
            LegacyRegistryContract::stakerURLsCall::new((address,)),
            self.config.address,
        )
        .await?;

        Ok(Maker::new(address, url._0))
    }

    async fn get_makers(&self) -> Result<Vec<Maker>, RegistryError> {
        let makers =
            get_makers_events::<LegacyRegistryContract::SetURL, _, _>(&self.provider, &self.config)
                .await?
                .into_iter()
                .map(|e| normalized_maker(e.account, e.url))
                .collect();

        Ok(makers)
    }

    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        let x = call(
            &self.provider,
            LegacyRegistryContract::getSupportedTokensCall::new((maker_address,)),
            self.config.address,
        )
        .await?;

        Ok(x.tokenList)
    }
}

pub struct RegistryV4<T, N> {
    provider: Arc<RootProvider<T, N>>,
    config: RegistryConfig,
}

impl<T, N> RegistryV4<T, N> {
    pub fn new(provider: Arc<RootProvider<T, N>>, chain_id: u64, version: RegistryVersion) -> Self {
        let config = RegistryConfig::new(chain_id, version);

        Self { provider, config }
    }
}

#[async_trait]
impl<T, N> RegistryContract for RegistryV4<T, N>
where
    T: Transport + Clone + Send + Sync,
    N: Network + Send + Sync,
{
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        let url = call(
            &self.provider,
            RegistryV4Contract::stakerServerURLsCall::new((address,)),
            self.config.address,
        )
        .await?;

        Ok(Maker::new(address, url._0))
    }

    async fn get_makers(&self) -> Result<Vec<Maker>, RegistryError> {
        let makers = get_makers_events::<RegistryV4Contract::SetServerURL, _, _>(
            &self.provider,
            &self.config,
        )
        .await?
        .into_iter()
        .map(|e| normalized_maker(e.account, e.url))
        .collect();

        Ok(makers)
    }

    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        let x = call(
            &self.provider,
            RegistryV4Contract::getTokensForStakerCall::new((maker_address,)),
            self.config.address,
        )
        .await?;

        Ok(x.tokenList)
    }
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("Invalid log")]
    Log,
    #[error(transparent)]
    Sol(#[from] alloy::sol_types::Error),
}
