use alloy::primitives::B256;
use serde::Deserialize;

use super::vote::Vote;

#[derive(Debug, Deserialize)]
pub struct GroupedProposal {
    pub root: B256,
    pub votes: Vec<Vote>,
}
