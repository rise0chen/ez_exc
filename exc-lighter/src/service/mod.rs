mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use crate::response::FullHttpResponse;
use core::time::Duration;
use exc_util::error::ExchangeError;
use exc_util::http::Client;
use exc_util::interface::{ApiKind, Rest};
use futures::future::{ready, BoxFuture};
use futures::{FutureExt, TryFutureExt};
use lighter_rs::client::TxClient;
use lighter_rs::ws_client::WsClient;
use std::sync::Arc;
use tower::{Service, ServiceBuilder};

/// Lighter API.
#[derive(Clone)]
pub struct Lighter {
    key: Key,
    http: Client,
    tx: Arc<TxClient>,
    ws: Arc<WsClient>,
}
impl Lighter {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default().service(Client::new(None));
        let tx = TxClient::new("https://mainnet.zklighter.elliot.ai", &key.key, key.account_index, key.key_index, 304).unwrap();
        let ws = WsClient::builder()
            .host("mainnet.zklighter.elliot.ai")
            .auth(key.read.as_str())
            .accounts(vec![key.account_index])
            .markets(vec![key.market_index])
            .order_books(vec![key.market_index])
            .build()
            .unwrap();
        Self {
            key,
            http,
            tx: Arc::new(tx),
            ws: Arc::new(ws),
        }
    }
    pub fn run(&self) {
        let ws = self.ws.clone();
        tokio::spawn(async move {
            loop {
                let ret = ws.run().await;
                ws.clear().await;
                tracing::info!("lighter ws exit: {ret:?}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}

impl<Req: Rest> Service<Req> for Lighter {
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
        tracing::trace!(?req, "http request;");
        match req {
            Ok(req) => self
                .http
                .call(req)
                .map_err(ExchangeError::from)
                .and_then(|resp| {
                    tracing::trace!("http response; status: {:?}", resp.status());
                    resp.bytes().map_err(|err| ExchangeError::UnexpectedResponseType(err.to_string()))
                })
                .and_then(|bytes| {
                    let resp = match serde_json::from_slice::<Req::Response>(&bytes) {
                        Ok(res) => Ok(res),
                        Err(_) => serde_json::from_slice::<FullHttpResponse<Req::Response>>(&bytes)
                            .map_err(|_| ExchangeError::UnexpectedResponseType(String::from_utf8_lossy(&bytes).into_owned()))
                            .and_then(|x| x.into()),
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
