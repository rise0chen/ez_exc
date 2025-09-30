use super::Okx;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Okx {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::book::GetDepthRequest;
        let req = GetDepthRequest {
            inst_id: symbol_id,
            sz: limit,
        };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| Depth {
            bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
            ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
            price: (resp.asks[0].0 + resp.bids[0].0) / 2.0,
            version: resp.ts,
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
