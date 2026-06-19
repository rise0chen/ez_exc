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
use exc_util::traits::*;
use futures_util::future::{BoxFuture, ready};
use futures_util::{FutureExt, TryFutureExt};
use std::sync::Arc;
use tower::ServiceExt;
use tower::{Service, ServiceBuilder};

/// Lbank API.
#[derive(Clone)]
pub struct Lbank {
    key: Key,
    http: Client,
    ws: Arc<crate::futures_web::ws::Ws>,
}

impl Lbank {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default().service(Client::new(None));
        let symbols = key.symbol.split(',');
        let symbols = symbols.filter_map(|x| if x.is_empty() { None } else { Some(x.to_owned()) }).collect();
        let ws = Arc::new(crate::futures_web::ws::Ws::new(symbols));
        Self { key, http, ws }
    }
    pub fn run(&mut self) {
        if self.ws.symbols.is_empty() {
            return;
        }
        let symbols = self.ws.symbols.clone().into_iter();
        let price_tick: Vec<_> = symbols
            .map(|s| {
                tokio::task::block_in_place(|| {
                    let rt = tokio::runtime::Handle::current();
                    let mut failed_times = 0;

                    loop {
                        match rt.block_on(async {
                            use crate::futures_web::http::info::GetInfoRequest;
                            let req = GetInfoRequest {
                                product_group: "SwapU",
                                instrument: s.clone(),
                            };
                            self.oneshot(req).await.map(|x| x.price_tick)
                        }) {
                            Ok(tick) => {
                                break tick;
                            }
                            Err(_) => {
                                failed_times += 1;
                                rt.block_on(tokio::time::sleep(Duration::from_secs(5 * failed_times)))
                            }
                        }
                    }
                })
            })
            .collect();
        let ws = self.ws.clone();
        tokio::spawn(async move {
            loop {
                let ret = ws.run(&price_tick).await;
                ws.clear();
                tracing::info!("lbank ws exit: {ret:?}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}

impl<Req: Rest> Service<Req> for Lbank {
    type Response = Req::Response;
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.http.poll_ready(cx).map_err(ExchangeError::from)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let req = match req.api_kind() {
            ApiKind::FuturesWeb => crate::futures_web::http::req_to_http(&req, &self.key),
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
impl ExchangeTrait for Lbank {
    async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        self.get_balance().await
    }
    async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        self.get_positions(symbol).await
    }

    async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        self.perfect_symbol(symbol).await
    }

    async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        self.get_index_price(symbol).await
    }

    async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        self.get_funding_rate(symbol).await
    }

    async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        self.get_funding_rate_history(symbol, day).await
    }

    async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        self.get_st_rate(symbol).await
    }

    async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        self.get_depth(symbol, limit).await
    }

    async fn get_order(&mut self, id: OrderId) -> Result<Order, ExchangeError> {
        self.get_order(id).await
    }

    async fn place_order(&mut self, symbol: &Symbol, order_req: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        self.place_order(symbol, order_req).await
    }

    async fn cancel_order(&mut self, id: OrderId) -> Result<(), ExchangeError> {
        self.cancel_order(id).await
    }
}
