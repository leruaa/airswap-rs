use alloy_primitives::{Address, FixedBytes, U256, U64};
use serde::Deserialize;
use thiserror::Error;

use super::{Response, ResponseDecodeError, ResponseResult, ResultPayload, Signature};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPayload {
    pub nonce: U256,
    pub expiry: U256,
    pub signer_wallet: Address,
    pub signer_token: Address,
    pub signer_amount: U256,
    pub sender_token: Address,
    pub sender_amount: U256,
    pub sender_wallet: Option<Address>,
    pub signer_fee: Option<String>,
    pub swap_contract: Option<String>,
    pub signature: Option<Signature>,
    pub r: FixedBytes<32>,
    pub s: FixedBytes<32>,
    pub v: Option<U64>,
}

impl PartialEq for OrderPayload {
    fn eq(&self, other: &Self) -> bool {
        self.signer_amount == other.signer_amount
    }
}

impl Eq for OrderPayload {}

impl PartialOrd for OrderPayload {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderPayload {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.signer_amount.cmp(&other.signer_amount)
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct EncodeError(String);

impl TryFrom<Response> for OrderPayload {
    type Error = ResponseDecodeError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Result(res) => res.try_into(),
            Response::Error(err) => Err(ResponseDecodeError::Remote(err.error)),
        }
    }
}

impl TryFrom<ResponseResult> for OrderPayload {
    type Error = ResponseDecodeError;

    fn try_from(value: ResponseResult) -> Result<Self, Self::Error> {
        match value.result {
            ResultPayload::SignerSideOrder(signer_side_order) => Ok(*signer_side_order),
            ResultPayload::Error(err) => Err(ResponseDecodeError::Remote(*err)),
            _ => Err(ResponseDecodeError::WrongVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;

    use super::OrderPayload;

    #[test]
    fn sort() {
        let mut orders = vec![
            OrderPayload {
                signer_amount: U256::from(5000),
                ..Default::default()
            },
            OrderPayload {
                signer_amount: U256::from(10000),
                ..Default::default()
            },
            OrderPayload {
                signer_amount: U256::from(3000),
                ..Default::default()
            },
        ];

        orders.sort();

        let best_order = orders.last().unwrap();

        assert_eq!(best_order.signer_amount.to_string(), "10000")
    }
}
