use serde::Deserialize;

use super::{Pricing, Response, ResponseDecodeError, ResponseResult, ResultPayload};

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct PricingPayload(pub Vec<Pricing>);

impl TryFrom<Response> for PricingPayload {
    type Error = ResponseDecodeError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Result(res) => res.try_into(),
            Response::Error(err) => Err(ResponseDecodeError::Remote(err.error)),
            Response::Unknown(value) => Err(ResponseDecodeError::UnknownVariant(value.to_string())),
        }
    }
}

impl TryFrom<ResponseResult> for PricingPayload {
    type Error = ResponseDecodeError;

    fn try_from(value: ResponseResult) -> Result<Self, Self::Error> {
        match value.result {
            ResultPayload::Pricing(pricings) => Ok(*pricings),
            _ => Err(ResponseDecodeError::WrongVariant),
        }
    }
}
