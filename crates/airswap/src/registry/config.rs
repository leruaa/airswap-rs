use std::{fmt::Display, str::FromStr};

use alloy::primitives::{address, Address, BlockNumber};

pub struct RegistryConfig {
    pub address: Address,
    pub from_block: BlockNumber,
}

impl RegistryConfig {
    pub fn new(chain_id: u64, version: RegistryVersion) -> Self {
        match (chain_id, version) {
            (1, RegistryVersion::Legacy) => Self {
                address: address!("8F9DA6d38939411340b19401E8c54Ea1f51B8f95"),
                from_block: 12782029,
            },
            (1, RegistryVersion::V4) => Self {
                address: address!("f5E6730c5A915b6f47AeAB0952655036aE2e73E9"),
                from_block: 17215828,
            },
            (5, RegistryVersion::Legacy) => Self {
                address: address!("05545815a5579d80Bd4c380da3487EAC2c4Ce299"),
                from_block: 6537104,
            },
            (5, RegistryVersion::V4) => Self {
                address: address!("6787cD07B0E6934BA9c3D1eBf3866eF091697128"),
                from_block: 8845816,
            },
            (137, RegistryVersion::Legacy) => Self {
                address: address!("9F11691FA842856E44586380b27Ac331ab7De93d"),
                from_block: 26036024,
            },
            (42161, RegistryVersion::V4) => Self {
                address: address!("e30E9c001dEFb5F0B04fD21662454A2427F4257A"),
                from_block: 178078567,
            },
            (chain_id, version) => unimplemented!("Chain {chain_id} {version:?} not supported"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum RegistryVersion {
    #[default]
    Legacy,
    V4,
}

impl Display for RegistryVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryVersion::Legacy => write!(f, "legacy"),
            RegistryVersion::V4 => write!(f, "v4"),
        }
    }
}

impl FromStr for RegistryVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "legacy" => Ok(RegistryVersion::Legacy),
            "v4" => Ok(RegistryVersion::V4),
            other => Err(format!("The registry '{other}' is not supported")),
        }
    }
}
