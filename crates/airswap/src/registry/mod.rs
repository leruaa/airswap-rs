mod client;
mod config;
mod maker;
mod maker_with_supported_tokens;

use std::collections::HashMap;

use alloy::primitives::{address, Address};
pub use client::{RegistryClient, RegistryError};
pub use config::RegistryVersion;
pub use maker::Maker;
pub use maker_with_supported_tokens::MakerWithSupportedTokens;
use once_cell::sync::Lazy;

pub static KNOWN_MAKERS: Lazy<HashMap<Address, String>> = Lazy::new(|| {
    HashMap::from([
        (
            address!("143395428158a57d17bcd8899770460656de98e4"),
            String::from("MyEth"),
        ),
        (
            address!("111bb8c3542f2b92fb41b8d913c01d3788431111"),
            String::from("B2C2"),
        ),
        (
            address!("bb289bc97591f70d8216462df40ed713011b968a"),
            String::from("Alphalab"),
        ),
        (
            address!("e0d90babe0081cf34328270620cd127eab8073db"),
            String::from("Altono"),
        ),
        (
            address!("0F4A4B5A9935544190a6eAf34ec5A343738D4166"),
            String::from("N"),
        ),
    ])
});
