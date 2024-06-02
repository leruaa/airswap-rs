use std::{env, sync::Arc};

use airswap::{RegistryClient, RegistryVersion};
use alloy::primitives::{address, Address};
use alloy::providers::ProviderBuilder;
use dotenv::dotenv;

pub static MYETH: Address = address!("143395428158a57d17bcd8899770460656de98e4");

#[tokio::test]
async fn test_registry() {
    dotenv().ok();

    let eth_rpc = env::var("ETH_RPC_URL").unwrap();
    let provider = ProviderBuilder::new().on_http(eth_rpc.parse().unwrap());

    let registry_client = RegistryClient::new(Arc::new(provider), 1, RegistryVersion::Legacy);

    let maker = registry_client.get_maker(MYETH).await.unwrap();

    assert_eq!(maker.address, MYETH);
}
