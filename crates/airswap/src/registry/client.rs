use std::collections::HashSet;
use std::marker::PhantomData;
use std::vec;

use alloy::primitives::Address;
use alloy::sol_types::SolCall;
use alloy::{
    network::{Network, TransactionBuilder},
    providers::Provider,
    sol,
    transports::TransportError,
};
use async_trait::async_trait;
use futures::{future::try_join_all, TryFutureExt};
use thiserror::Error;

use crate::{Config, Maker, MakerWithSupportedTokens, ProtocolVersion};

sol!(LegacyRegistryContract, "abi/registry.json");
sol!(RegistryV4Contract, "abi/registry_v4.json");

#[async_trait]
pub trait RegistryContract {
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError>;
    async fn get_makers(&self, token: Address) -> Result<HashSet<Maker>, RegistryError>;
    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError>;
}

async fn call<P, N, C>(provider: &P, call: C, to: Address) -> Result<C::Return, RegistryError>
where
    P: Provider<N>,
    N: Network,
    C: SolCall + Send + Sync,
{
    let tx = N::TransactionRequest::default()
        .with_input(call.abi_encode())
        .with_to(to);

    let result = provider.call(tx).await?;
    let decoded = C::abi_decode_returns(&result)?;

    Ok(decoded)
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

#[derive(Clone)]
pub enum RegistryClient<P, N> {
    Legacy(LegacyRegistry<P, N>),
    V4(RegistryV4<P, N>),
}

impl<P, N> RegistryClient<P, N>
where
    P: Provider<N>,
    N: Network,
{
    pub fn new(provider: P, config: Config) -> Self {
        match config.protocol_version {
            ProtocolVersion::Legacy => Self::Legacy(LegacyRegistry::new(provider, config)),
            _ => Self::V4(RegistryV4::new(provider, config)),
        }
    }

    pub async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        match self {
            RegistryClient::Legacy(registry) => registry.get_maker(address).await,
            RegistryClient::V4(registry) => registry.get_maker(address).await,
        }
    }

    pub async fn get_maker_with_supported_tokens(
        &self,
        address: Address,
    ) -> Result<MakerWithSupportedTokens, RegistryError> {
        let maker = self.get_maker(address).await?;
        let supported_tokens = self.get_tokens(maker.address).await?;

        Ok(MakerWithSupportedTokens::new(maker, supported_tokens))
    }

    pub async fn get_makers(&self, token: Address) -> Result<HashSet<Maker>, RegistryError> {
        match self {
            RegistryClient::Legacy(registry) => registry.get_makers(token).await,
            RegistryClient::V4(registry) => registry.get_makers(token).await,
        }
    }

    pub async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        match self {
            RegistryClient::Legacy(registry) => registry.get_tokens(maker_address).await,
            RegistryClient::V4(registry) => registry.get_tokens(maker_address).await,
        }
    }

    pub async fn get_makers_with_supported_tokens(
        &self,
        token: Address,
    ) -> Result<Vec<MakerWithSupportedTokens>, RegistryError> {
        let futures = self.get_makers(token).await?.into_iter().map(|m| {
            self.get_tokens(m.address)
                .map_ok(|supported_tokens| MakerWithSupportedTokens::new(m, supported_tokens))
        });

        let makers_with_supported_tokens = try_join_all(futures).await?;

        Ok(makers_with_supported_tokens)
    }
}

#[derive(Clone)]
pub struct LegacyRegistry<P, N> {
    provider: P,
    config: Config,
    phantom: PhantomData<N>,
}

impl<P, N> LegacyRegistry<P, N> {
    pub fn new(provider: P, config: Config) -> Self {
        Self {
            provider,
            config,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<P, N> RegistryContract for LegacyRegistry<P, N>
where
    P: Provider<N>,
    N: Network + Send + Sync,
{
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        let url = call(
            &self.provider,
            LegacyRegistryContract::stakerURLsCall::new((address,)),
            self.config.registry_address,
        )
        .await?;

        Ok(Maker::new(address, url))
    }

    async fn get_makers(&self, token: Address) -> Result<HashSet<Maker>, RegistryError> {
        let mut makers = HashSet::new();

        let maker_addresses = call(
            &self.provider,
            LegacyRegistryContract::getStakersForTokenCall::new((token,)),
            self.config.registry_address,
        )
        .await?;

        for a in maker_addresses {
            let urls = call(
                &self.provider,
                LegacyRegistryContract::getURLsForStakersCall::new((vec![a],)),
                self.config.registry_address,
            )
            .await?;

            makers.insert(normalized_maker(a, urls[0].clone()));
        }

        Ok(makers)
    }

    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        let tokens = call(
            &self.provider,
            LegacyRegistryContract::getSupportedTokensCall::new((maker_address,)),
            self.config.registry_address,
        )
        .await?;

        Ok(tokens)
    }
}

#[derive(Clone)]
pub struct RegistryV4<P, N> {
    provider: P,
    config: Config,
    phantom: PhantomData<N>,
}

impl<P, N> RegistryV4<P, N> {
    pub fn new(provider: P, config: Config) -> Self {
        Self {
            provider,
            config,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<P, N> RegistryContract for RegistryV4<P, N>
where
    P: Provider<N>,
    N: Network + Send + Sync,
{
    async fn get_maker(&self, address: Address) -> Result<Maker, RegistryError> {
        let url = call(
            &self.provider,
            RegistryV4Contract::stakerServerURLsCall::new((address,)),
            self.config.registry_address,
        )
        .await?;

        Ok(Maker::new(address, url))
    }

    async fn get_makers(&self, token: Address) -> Result<HashSet<Maker>, RegistryError> {
        let mut makers = HashSet::new();

        let maker_addresses = call(
            &self.provider,
            RegistryV4Contract::getStakersForTokenCall::new((token,)),
            self.config.registry_address,
        )
        .await?;

        for a in maker_addresses {
            let urls = call(
                &self.provider,
                RegistryV4Contract::getServerURLsForStakersCall::new((vec![a],)),
                self.config.registry_address,
            )
            .await?;

            makers.insert(normalized_maker(a, urls[0].clone()));
        }

        Ok(makers)
    }

    async fn get_tokens(&self, maker_address: Address) -> Result<Vec<Address>, RegistryError> {
        let tokens = call(
            &self.provider,
            RegistryV4Contract::getTokensForStakerCall::new((maker_address,)),
            self.config.registry_address,
        )
        .await?;

        Ok(tokens)
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
