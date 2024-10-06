use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Protocol {
    pub interface_id: String,
    pub params: ProtocolParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProtocolParams {
    pub chain_id: String,
    pub swap_contract_address: String,
    pub wallet_address: String,
}
