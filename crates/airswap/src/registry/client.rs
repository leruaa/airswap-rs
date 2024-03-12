use std::sync::Arc;

use alloy_network::{Network, TransactionBuilder};
use alloy_primitives::Address;
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::Filter;
use alloy_sol_types::{sol, SolCall, SolEvent};
use alloy_transport::{Transport, TransportError};
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

async fn call<C, N, T>(
    provider: &Arc<RootProvider<N, T>>,
    call: C,
    to: Address,
) -> Result<C::Return, RegistryError>
where
    C: SolCall + Send + Sync,
    N: Network,
    T: Transport + Clone,
{
    let tx = N::TransactionRequest::default()
        .with_input(call.abi_encode().into())
        .with_to(to.into());

    let result = provider.call(&tx, None).await?;
    let decoded = C::abi_decode_returns(&result, true)?;

    Ok(decoded)
}

async fn get_makers_events<E, N, T>(
    provider: &Arc<RootProvider<N, T>>,
    config: &RegistryConfig,
) -> Result<Vec<E>, RegistryError>
where
    E: SolEvent,
    N: Network,
    T: Transport + Clone,
{
    let filter = Filter::new()
        .from_block(config.from_block)
        .address(config.address)
        .event(E::SIGNATURE);

    let set_url_events = provider.get_logs(filter).await?;

    set_url_events
        .into_iter()
        .map(|log| {
            alloy_primitives::Log::new(config.address, log.topics, log.data)
                .ok_or(RegistryError::Log)
                .and_then(|log| E::decode_log(&log, true).map_err(RegistryError::from))
                .map(|l| l.data)
        })
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
    pub fn new<N, T>(
        provider: Arc<RootProvider<N, T>>,
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
impl<N, T> RegistryContract for LegacyRegistry<N, T>
where
    N: Network + Send + Sync,
    T: Transport + Clone + Send + Sync,
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

pub struct RegistryV4<N, T> {
    provider: Arc<RootProvider<N, T>>,
    config: RegistryConfig,
}

impl<N, T> RegistryV4<N, T> {
    pub fn new(provider: Arc<RootProvider<N, T>>, chain_id: u64, version: RegistryVersion) -> Self {
        let config = RegistryConfig::new(chain_id, version);

        Self { provider, config }
    }
}

#[async_trait]
impl<N, T> RegistryContract for RegistryV4<N, T>
where
    N: Network + Send + Sync,
    T: Transport + Clone + Send + Sync,
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
    Sol(#[from] alloy_sol_types::Error),
}
