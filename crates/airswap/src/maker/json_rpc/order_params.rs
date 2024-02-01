use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderParams {
    pub chain_id: String,      // ID of the chain intended for use
    pub swap_contract: String, // Swap contract intended for use
    pub signer_token: String,  // Token the signer would transfer
    pub sender_token: String,  // Token the sender would transfer
    pub sender_wallet: String, // Wallet of the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>, // Requested order expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxying_for: Option<String>, // Ultimate counterparty of the swap (Optional)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignerSideOrderParams {
    pub sender_amount: String, // Amount the client want to sell to the maker
    #[serde(flatten)]
    pub order: OrderParams,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderSideOrderParams {
    pub signer_amount: String, // Amount the client want to buy from the maker
    #[serde(flatten)]
    pub order: OrderParams,
}
