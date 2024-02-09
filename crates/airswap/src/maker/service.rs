use std::{
    future::ready,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use bigdecimal::BigDecimal;
use futures::{future::Either, Future, TryFutureExt};
use reqwest::Client as HttpClient;
use tower::{filter::Predicate, Service};

use crate::{
    json_rpc::{Payload, Request, Response, ResponseDecodeError, ResponseResult},
    MakerWithSupportedTokens,
};

use super::MakerError;

pub struct MakerService {
    maker: MakerWithSupportedTokens,
    client: HttpClient,
}

impl MakerService {
    pub fn new(maker: MakerWithSupportedTokens) -> Self {
        Self {
            maker,
            client: HttpClient::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    fn can_handle(&self, payload: &Payload) -> bool {
        match payload {
            Payload::SignerSideOrder(params) => self
                .maker
                .can_handle(&[params.order.sender_token, params.order.signer_token]),
            Payload::SenderSideOrder(params) => self
                .maker
                .can_handle(&[params.order.sender_token, params.order.signer_token]),
            Payload::Pricing(_) => true,
            Payload::AllPricing(_) => true,
        }
    }

    pub fn maker(&self) -> &MakerWithSupportedTokens {
        &self.maker
    }
}

impl Service<Payload> for MakerService {
    type Response = ResponseResult;

    type Error = MakerError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, payload: Payload) -> Self::Future {
        if !self.can_handle(&payload) {
            return Box::pin(ready(Err(MakerError::PairNotSupported)));
        }

        let fut = self
            .client
            .post(self.maker.url())
            .json(&Request::from(payload))
            .send()
            .map_err(Into::into)
            .and_then(|resp| match resp.status().as_u16() {
                204 => Either::Left(ready(Err(MakerError::EmptyResponse))),
                s if s >= 400 => Either::Left(ready(Err(MakerError::ServerError(resp.status())))),
                _ => Either::Right(
                    resp.json::<Response>()
                        .map_ok(|r| match r {
                            Response::Result(result) => Ok(result),
                            Response::Error(err) => match err.error.code {
                                -33605 => Err(MakerError::RateLimitMet),
                                _ => Err(ResponseDecodeError::Remote(err.error).into()),
                            },
                            Response::Unknown(value) => {
                                Err(ResponseDecodeError::UnknownVariant(value.to_string()).into())
                            }
                        })
                        .unwrap_or_else(|err| Err(MakerError::from(err))),
                ),
            });

        Box::pin(fut)
    }
}

pub struct Threshold<P> {
    value: BigDecimal,
    phantom: PhantomData<P>,
}

impl<P> Threshold<P> {
    pub fn new(value: BigDecimal) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<P> Predicate<(P, BigDecimal)> for Threshold<P> {
    type Request = P;

    fn check(
        &mut self,
        (payload, amount): (P, BigDecimal),
    ) -> Result<Self::Request, tower::BoxError> {
        if self.value < amount {
            Ok(payload)
        } else {
            Err(Box::new(MakerError::AmountTooLow(amount)))
        }
    }
}
