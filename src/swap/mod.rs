use std::sync::Arc;

use alloy_primitives::Address;
use alloy_providers::provider::{Provider, TempProvider};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport::{BoxTransport, TransportError};
use thiserror::Error;

sol!(SwapERC20Contract, "abi/swap_erc20.json");

pub async fn get_swap_events<B: Into<BlockNumberOrTag>>(
    provider: Arc<Provider<BoxTransport>>,
    swap_address: Address,
    from_block: B,
    to_block: Option<B>,
) -> Result<Vec<SwapERC20Contract::SwapERC20>, SwapError> {
    let filter = Filter::new()
        .from_block(from_block)
        .to_block(to_block.map(|b| b.into()).unwrap_or_default())
        .address(swap_address)
        .event(SwapERC20Contract::SwapERC20::SIGNATURE);

    let swap_event_logs = provider.get_logs(filter).await?;
    let mut events = vec![];

    for log in swap_event_logs.into_iter().filter(|l| !l.removed) {
        let swap_event = SwapERC20Contract::SwapERC20::decode_log_data(&log.try_into()?, true)?;

        events.push(swap_event);
    }

    Ok(events)
}

#[derive(Error, Debug)]
pub enum SwapError {
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error(transparent)]
    Log(#[from] alloy_rpc_types::LogError),
    #[error(transparent)]
    Sol(#[from] alloy_sol_types::Error),
}
