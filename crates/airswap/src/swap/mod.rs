use std::sync::Arc;

use alloy_json_rpc::{ErrorPayload, Id, Request, RequestMeta, ResponsePayload};
use alloy_primitives::{Address, U256};
use alloy_providers::provider::{Provider, TempProvider};
use alloy_pubsub::PubSubFrontend;
use alloy_rpc_types::{
    pubsub::{Params, SubscriptionKind},
    BlockNumberOrTag, Filter, Log,
};
use alloy_sol_types::{sol, SolEvent};
use alloy_transport::{BoxTransport, TransportError};
use futures::{
    stream::{self, BoxStream},
    StreamExt, TryStreamExt,
};
use thiserror::Error;
use tracing::error;

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

pub async fn get_swap_events_stream(
    front_end: &PubSubFrontend,
    swap_address: Address,
    id: Id,
) -> Result<BoxStream<Result<SwapERC20Contract::SwapERC20, SwapError>>, SwapError> {
    let stringified_id = id.to_string();
    let req = Request {
        meta: RequestMeta::new("eth_subscribe", id),
        params: [
            serde_json::to_value(SubscriptionKind::Logs)?,
            serde_json::to_value(Params::Logs(Box::new(
                Filter::new()
                    .address(swap_address)
                    .event_signature(SwapERC20Contract::SwapERC20::SIGNATURE_HASH),
            )))?,
        ],
    };

    let response = front_end
        .send(req.serialize()?)
        .await?
        .deser_success::<U256>()
        .unwrap();

    let subscription_id = match response.payload {
        ResponsePayload::Success(subscription_id) => Ok(subscription_id),
        ResponsePayload::Failure(err) => Err(SwapError::Payload(err)),
    }?;

    let rx = front_end.get_subscription(subscription_id).await?;

    let stream = stream::unfold(
        (rx, stringified_id),
        |(mut rx, stringified_id)| async move {
            match rx.recv().await {
                Ok(value) => Some((value, (rx, stringified_id))),
                Err(err) => {
                    error!("Subscription {stringified_id} ended: {err}");
                    None
                }
            }
        },
    );

    let stream = stream
        .map(|value| serde_json::from_str::<Log>(value.get()).map_err(Into::into))
        .and_then(|log| async {
            SwapERC20Contract::SwapERC20::decode_log_data(&log.try_into()?, true)
                .map_err(Into::into)
        });

    Ok(stream.boxed())
}

#[derive(Error, Debug)]
pub enum SwapError {
    #[error("{0}")]
    Payload(ErrorPayload),
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error(transparent)]
    Log(#[from] alloy_rpc_types::LogError),
    #[error(transparent)]
    Sol(#[from] alloy_sol_types::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Receive error")]
    Receive,
}
