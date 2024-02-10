use std::sync::Arc;

use airswap::{Maker, MakerClient, MakerWithSupportedTokens};
use alloy_primitives::{address, U256};

#[tokio::test]
async fn test_maker() {
    let maker = MakerWithSupportedTokens {
        maker: Maker {
            address: address!("bb289bc97591f70d8216462df40ed713011b968a"),
            url: String::from("https://airswap-pmm-rfq-server-i24uhfiu3fh.alphalab.cc"),
        },
        supported_tokens: vec![
            address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
            address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            address!("dac17f958d2ee523a2206206994597c13d831ec7"),
            address!("2260fac5e5542a773aa44fbcfedf7c193bc2c599"),
            address!("5a98fcbea516cf06857215779fd812ca3bef1b32"),
            address!("4d224452801aced8b2f0aebe155379bb5d594381"),
            address!("d533a949740bb3306d119cc777fa900ba034cd52"),
            address!("514910771af9ca656af840dff83e8264ecf986ca"),
            address!("7d1afa7b718fb893db30a3abc0cfc608aacfebb0"),
            address!("6b3595068778dd592e39a122f4f5a5cf09c90fe2"),
            address!("6b175474e89094c44da98b954eedeac495271d0f"),
            address!("15d4c048f83bd7e37d49ea4c83a07267ec4203da"),
            address!("3845badade8e6dff049820680d1f14bd3903a5d0"),
            address!("1f9840a85d5af5bf1d1762f925bdaddc4201f984"),
        ],
    };

    let maker_client = MakerClient::new(1, Arc::new(maker));
    let amount = U256::from(2000000000);

    let payload = maker_client
        .get_sell_quote(
            address!("EdCb63f859905Be353D85D53041E9697Dbea5f81"),
            address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            amount,
        )
        .await
        .unwrap();

    assert_eq!(payload.sender_amount, amount);
}
