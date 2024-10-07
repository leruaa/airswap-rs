use std::{fmt::Display, str::FromStr};

use alloy::primitives::{address, Address, BlockNumber};

#[derive(Debug, Clone)]
pub struct Config {
    pub registry_address: Address,
    pub registry_from_block: BlockNumber,
    pub swap_address: Address,
    pub protocol_version: ProtocolVersion,
}

impl Config {
    pub const fn new(chain_id: u64, protocol_version: ProtocolVersion) -> Self {
        let (registry_address, registry_from_block) = match protocol_version {
            ProtocolVersion::Legacy => match chain_id {
                1 => (
                    address!("8F9DA6d38939411340b19401E8c54Ea1f51B8f95"),
                    12782029,
                ),
                137 => (
                    address!("9F11691FA842856E44586380b27Ac331ab7De93d"),
                    26036024,
                ),
                _ => unimplemented!(),
            },
            ProtocolVersion::V4 | ProtocolVersion::V5 => (
                address!("e30E9c001dEFb5F0B04fD21662454A2427F4257A"),
                match chain_id {
                    1 => 12782029,
                    137 => 53161197,
                    42161 => 178078567,
                    _ => unimplemented!(),
                },
            ),
        };

        let swap_address = match protocol_version {
            ProtocolVersion::Legacy => match chain_id {
                1 => address!("522d6f36c95a1b6509a14272c17747bbb582f2a6"),
                137 => address!("6713c23261c8a9b7d84dd6114e78d9a7b9863c1a"),
                _ => unimplemented!(),
            },
            ProtocolVersion::V4 => address!("d82FA167727a4dc6D6F55830A2c47aBbB4b3a0F8"),
            ProtocolVersion::V5 => address!("D82E10B9A4107939e55fCCa9B53A9ede6CF2fC46"),
        };

        Self {
            registry_address,
            registry_from_block,
            swap_address,
            protocol_version,
        }
    }

    pub const fn mainnet_legacy() -> Self {
        Self::new(1, ProtocolVersion::Legacy)
    }

    pub const fn mainnet_v4() -> Self {
        Self::new(1, ProtocolVersion::V4)
    }

    pub const fn mainnet_v5() -> Self {
        Self::new(1, ProtocolVersion::V5)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum ProtocolVersion {
    #[default]
    Legacy,
    V4,
    V5,
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolVersion::Legacy => write!(f, "legacy"),
            ProtocolVersion::V4 => write!(f, "v4"),
            ProtocolVersion::V5 => write!(f, "v5"),
        }
    }
}

impl FromStr for ProtocolVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "legacy" => Ok(ProtocolVersion::Legacy),
            "v4" => Ok(ProtocolVersion::V4),
            "v5" => Ok(ProtocolVersion::V5),
            other => Err(format!("The version '{other}' is not supported")),
        }
    }
}
