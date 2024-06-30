use std::fmt::Display;

use alloy::{
    primitives::{keccak256, Address, B256, U256},
    sol,
    sol_types::SolValue,
};
use serde::Deserialize;

sol! {
    #[derive(Debug, Deserialize)]
    struct Vote {
        address address;
        uint256 points;
    }
}

impl Vote {
    pub fn new(address: Address, points: U256) -> Self {
        Self { address, points }
    }
}

impl Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.address, self.points)
    }
}

impl From<&Vote> for B256 {
    fn from(value: &Vote) -> Self {
        keccak256(value.abi_encode_packed())
    }
}
