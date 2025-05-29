mod account;
mod book;
mod info;
mod trading;

use crate::key::Key;
use crate::response::FullHttpResponse;
use exc_core::transport::http::Client;
use exc_core::ExchangeError;
use exc_util::interface::{ApiKind, Rest};
use futures::future::{ready, BoxFuture};
use futures::{FutureExt, TryFutureExt};
use tower::{Service, ServiceBuilder};

/// Gate API.
#[derive(Clone)]
pub struct Gate {
    key: Key,
    http: Client,
}

impl Gate {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default().service(Client::new());
        Self { key, http }
    }
}

impl<Req: Rest> Service<Req> for Gate {
    type Response = Req::Response;
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.http.poll_ready(cx).map_err(ExchangeError::from)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let req = match req.api_kind() {
            ApiKind::FuturesApi => crate::futures_api::http::req_to_http(&req, &self.key),
            _ => unreachable!(),
        };
        match req {
            Ok(req) => self
                .http
                .call(req)
                .map_err(ExchangeError::from)
                .and_then(|resp| {
                    trace!("http response; status: {:?}", resp.status());
                    resp.bytes().map_err(|err| ExchangeError::Other(err.into()))
                })
                .and_then(|bytes| {
                    let resp = match serde_json::from_slice::<FullHttpResponse<Req::Response>>(&bytes) {
                        Ok(res) => res.into(),
                        Err(_) => serde_json::from_slice::<Req::Response>(&bytes)
                            .map_err(|_| ExchangeError::UnexpectedResponseType(String::from_utf8_lossy(&bytes).into_owned())),
                        //.map_err(|e| ExchangeError::Other(e.into())),
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
