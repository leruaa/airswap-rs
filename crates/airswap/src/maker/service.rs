use std::{
    future::ready,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::{future::Either, Future, TryFutureExt};
use reqwest::Client as HttpClient;
use tower::Service;

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

    pub fn can_handle(&self, payload: &Payload) -> bool {
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
}

impl Service<Payload> for MakerService {
    type Response = ResponseResult;

    type Error = MakerError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, payload: Payload) -> Self::Future {
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
