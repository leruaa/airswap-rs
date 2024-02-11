mod client;
mod config;
mod error;
pub mod json_rpc;
mod service;

use alloy_primitives::{Address, U256};
pub use client::MakerClient;
pub use config::MakerConfig;
pub use error::MakerError;
pub use service::{MakerService, ThresholdLayer};

use self::json_rpc::{OrderParams, SenderSideOrderParams, SignerSideOrderParams};

pub fn build_buy_order(
    from: Address,
    from_token: Address,
    to_token: Address,
    amount: U256,
    swap_address: Address,
    chain_id: u64,
) -> SenderSideOrderParams {
    SenderSideOrderParams {
        signer_amount: amount.to_string(),
        order: OrderParams {
            chain_id: chain_id.to_string(),
            signer_token: to_token,   // Token the signer would transfer
            sender_token: from_token, // Token the sender would transfer
            sender_wallet: format!("{:?}", from), // Wallet of the sender
            swap_contract: format!("{:?}", swap_address), // Swap contract intended for use
            expiry: None,
            proxying_for: None, // Ultimate counterparty of the swap (Optional)
        },
    }
}

pub fn build_sell_order(
    from: Address,
    from_token: Address,
    to_token: Address,
    amount: U256,
    swap_address: Address,
    chain_id: u64,
) -> SignerSideOrderParams {
    SignerSideOrderParams {
        sender_amount: amount.to_string(),
        order: OrderParams {
            chain_id: chain_id.to_string(),
            signer_token: to_token,   // Token the signer would transfer
            sender_token: from_token, // Token the sender would transfer
            sender_wallet: format!("{:?}", from), // Wallet of the sender
            swap_contract: format!("{:?}", swap_address), // Swap contract intended for use
            expiry: None,             // Ultimate counterparty of the swap (Optional)
            proxying_for: None,
        },
    }
}
