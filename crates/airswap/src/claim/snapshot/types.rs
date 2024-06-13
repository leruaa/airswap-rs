use std::{fmt::Display, ops::MulAssign};

use alloy::{
    primitives::{keccak256, Address, B256, U256},
    sol,
    sol_types::SolValue,
};
use bigdecimal::{BigDecimal, ToPrimitive};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ProposalInterval {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: Address,
    pub points: BigDecimal,
}

sol! {
    #[derive(Debug, Default)]
    struct ProposalGroup {
        bytes32[] ids;
    }

    #[derive(Debug)]
    struct AggregatedVote {
        address voter;
        uint256 points;
    }
}

impl ProposalGroup {
    pub fn new(mut ids: Vec<B256>) -> Self {
        ids.sort();

        Self { ids }
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn hash(&self) -> B256 {
        keccak256(self.abi_encode_packed())
    }
}

impl AggregatedVote {
    pub fn new(voter: Address, mut points: BigDecimal) -> Self {
        points.mul_assign(10000);

        Self {
            voter,
            points: U256::from(points.to_u64().unwrap()),
        }
    }
}

impl Display for AggregatedVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.voter, self.points)
    }
}
