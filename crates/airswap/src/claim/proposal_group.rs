use alloy::{
    primitives::{keccak256, B256},
    sol,
    sol_types::SolValue,
};
use itertools::Itertools;

sol! {
    struct ProposalGroup {
        bytes32[] ids;
    }
}

impl ProposalGroup {
    pub fn new(ids: Vec<B256>) -> Self {
        Self {
            ids: ids.into_iter().sorted().collect(),
        }
    }

    pub fn hash(&self) -> B256 {
        keccak256(self.abi_encode_packed())
    }
}
