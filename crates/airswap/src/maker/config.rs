use alloy_primitives::{address, Address};

pub struct MakerConfig {
    pub swap_address: Address,
}

impl MakerConfig {
    pub fn new(chain_id: u64) -> Self {
        match chain_id {
            1 => Self {
                swap_address: address!("d82FA167727a4dc6D6F55830A2c47aBbB4b3a0F8"),
            },
            5 => Self {
                swap_address: address!("d82FA167727a4dc6D6F55830A2c47aBbB4b3a0F8"),
            },
            137 => Self {
                swap_address: address!("d82FA167727a4dc6D6F55830A2c47aBbB4b3a0F8"),
            },
            42161 => Self {
                swap_address: address!("D82E10B9A4107939e55fCCa9B53A9ede6CF2fC46"),
            },
            chain_id => unimplemented!("Chain {chain_id} not supported"),
        }
    }
}
