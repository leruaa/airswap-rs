use serde::Deserialize;

use super::{Protocol, ResponseDecodeError, ResponseResult, ResultPayload};

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct ProtocolsPayload(pub Vec<Protocol>);

impl TryFrom<ResponseResult> for ProtocolsPayload {
    type Error = ResponseDecodeError;

    fn try_from(value: ResponseResult) -> Result<Self, Self::Error> {
        match value.result {
            ResultPayload::Protocols(protocols) => Ok(*protocols),
            ResultPayload::Error(error) => Err(ResponseDecodeError::Remote(*error)),
            _ => Err(ResponseDecodeError::WrongVariant),
        }
    }
}
