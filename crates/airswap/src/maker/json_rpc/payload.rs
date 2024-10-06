use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

use super::{PricingParams, SenderSideOrderParams, SignerSideOrderParams};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    #[serde(serialize_with = "empty_array")]
    Protocols,
    SignerSideOrder(SignerSideOrderParams),
    SenderSideOrder(SenderSideOrderParams),
    Pricing(PricingParams),
    AllPricing,
}

fn empty_array<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let seq = serializer.serialize_map(Some(0))?;
    seq.end()
}

#[cfg(test)]
mod tests {
    use super::Payload;

    #[test]
    fn serialize() {
        let payload = Payload::Protocols;

        let j = serde_json::to_string(&payload).unwrap();

        println!("{j}")
    }
}
