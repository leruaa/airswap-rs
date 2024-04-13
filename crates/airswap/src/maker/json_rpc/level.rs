use std::fmt::Display;

use alloy::primitives::U256;
use serde::{
    de::{Error as DeserializeError, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Number;

#[derive(Debug)]
pub struct Level {
    quantity: Number,
    price: Number,
}

impl Level {
    pub fn quantity(&self) -> f64 {
        self.quantity
            .as_f64()
            .ok_or("Can't convert quantity to f64".to_string())
            .unwrap()
    }

    pub fn price(&self) -> f64 {
        self.price
            .as_f64()
            .ok_or("Can't convert price to f64".to_string())
            .unwrap()
    }

    pub fn normalized_price(&self, decimals: u8) -> U256 {
        let normalized =
            (self.price.as_f64().unwrap_or(0_f64) * 10_f64.powi(decimals as i32)) as u64;

        U256::from(normalized)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.quantity, self.price)
    }
}

struct LevelVisitor;

impl<'de> Visitor<'de> for LevelVisitor {
    type Value = Level;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Level with a price and a quantity")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let quantity = seq
            .next_element::<Number>()?
            .ok_or(DeserializeError::missing_field("quantity"))?;

        let price = seq
            .next_element::<Number>()?
            .ok_or(DeserializeError::missing_field("price"))?;

        let level = Level { quantity, price };

        Ok(level)
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LevelVisitor)
    }
}
