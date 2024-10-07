use alloy::primitives::{Address, U256};
use tokio::sync::RwLock;
use tower::{Service, ServiceExt};

use crate::{
    build_buy_order,
    json_rpc::{OrderParams, Pair, PricingParams, PricingPayload, ResponseResult},
    Config, MakerWithSupportedTokens,
};

use super::{
    json_rpc::{OrderPayload, Payload, ProtocolsPayload, SignerSideOrderParams},
    MakerError, MakerService,
};

pub struct MakerClient {
    service: RwLock<MakerService>,
    chain_id: u64,
    config: Config,
}

impl MakerClient {
    pub fn new(chain_id: u64, maker: MakerWithSupportedTokens, config: Config) -> Self {
        Self {
            service: RwLock::new(MakerService::new(maker)),
            chain_id,
            config,
        }
    }

    pub async fn get_protocols(&self) -> Result<ProtocolsPayload, MakerError> {
        let payload = self.post(Payload::Protocols).await?.try_into()?;

        Ok(payload)
    }

    pub async fn get_buy_quote(
        &self,
        from: Address,
        from_token: Address,
        to_token: Address,
        amount: U256,
    ) -> Result<OrderPayload, MakerError> {
        let order = build_buy_order(
            from,
            from_token,
            to_token,
            amount,
            self.config.swap_address,
            self.chain_id,
        );

        let payload = self
            .post(Payload::SenderSideOrder(order))
            .await?
            .try_into()?;

        Ok(payload)
    }

    pub async fn get_sell_quote(
        &self,
        from: Address,
        from_token: Address,
        to_token: Address,
        amount: U256,
    ) -> Result<OrderPayload, MakerError> {
        let order = SignerSideOrderParams {
            sender_amount: amount.to_string(),
            order: OrderParams {
                chain_id: self.chain_id.to_string(),
                signer_token: to_token,   // Token the signer would transfer
                sender_token: from_token, // Token the sender would transfer
                sender_wallet: format!("{:?}", from), // Wallet of the sender
                swap_contract: format!("{:?}", self.config.swap_address), // Swap contract intended for use
                expiry: None, // Ultimate counterparty of the swap (Optional)
                proxying_for: None,
            },
        };

        let payload = self
            .post(Payload::SignerSideOrder(order))
            .await?
            .try_into()?;

        Ok(payload)
    }

    pub async fn get_pricing(&self, pairs: Vec<Pair>) -> Result<PricingPayload, MakerError> {
        let payload = self
            .post(Payload::Pricing(PricingParams::new(pairs)))
            .await?
            .try_into()?;

        Ok(payload)
    }

    pub async fn get_all_pricing(&self) -> Result<PricingPayload, MakerError> {
        let payload = self.post(Payload::AllPricing).await?.try_into()?;

        Ok(payload)
    }

    async fn post(&self, payload: Payload) -> Result<ResponseResult, MakerError> {
        let mut service = self.service.write().await;

        service.ready().await?.call(payload).await
    }
}
