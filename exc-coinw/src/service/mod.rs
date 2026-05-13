mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use crate::response::FullHttpResponse;
use exc_util::error::ExchangeError;
use exc_util::http::Client;
use exc_util::interface::{ApiKind, Rest};
use futures_util::future::{ready, BoxFuture};
use futures_util::{FutureExt, TryFutureExt};
use tower::{Service, ServiceBuilder};

/// Coinw API.
#[derive(Clone)]
pub struct Coinw {
    key: Key,
    http: Client,
}

impl Coinw {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default().service(Client::new(None));
        Self { key, http }
    }
}

impl<Req: Rest> Service<Req> for Coinw {
    type Response = Req::Response;
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.http.poll_ready(cx).map_err(ExchangeError::from)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let req = match req.api_kind() {
            ApiKind::FuturesApi => crate::futures_api::http::req_to_http(&req, &self.key),
            ApiKind::SpotApi => todo!(),
            _ => unreachable!(),
        };
        match req {
            Ok(req) => self
                .http
                .call(req)
                .map_err(ExchangeError::from)
                .and_then(|resp| resp.bytes().map_err(|err| ExchangeError::Other(err.into())))
                .and_then(|bytes| {
                    let resp = match serde_json::from_slice::<FullHttpResponse<Req::Response>>(&bytes) {
                        Ok(res) => res.into(),
                        Err(e) => Err(ExchangeError::Other(e.into())),
                    };
                    if resp.is_err() {
                        tracing::error!(?bytes, "http response;");
                    } else {
                        tracing::trace!(?bytes, "http response;");
                    }
                    ready(resp)
                })
                .boxed(),
            Err(err) => ready(Err(err.into())).boxed(),
        }
    }
}
