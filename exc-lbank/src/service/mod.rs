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
use exc_util::traits::*;
use futures_util::future::{BoxFuture, ready};
use futures_util::{FutureExt, TryFutureExt};
use tower::{Service, ServiceBuilder};

/// Lbank API.
#[derive(Clone)]
pub struct Lbank {
    key: Key,
    http: Client,
}

impl Lbank {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default().service(Client::new(None));
        Self { key, http }
    }
    pub fn run(&self) {}
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

    async fn cancel_order(&mut self, id: OrderId) -> Result<OrderId, ExchangeError> {
        self.cancel_order(id).await
    }
}
