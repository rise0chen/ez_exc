use super::Okx;
use exc_util::error::ExchangeError;
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
        let Some(resp) = resp else { return Err(ExchangeError::OrderNotFound) };
        let version = resp.ts;
        let mut bid: Vec<Order> = resp.bids.iter().map(|x| symbol.order(x.0, x.1)).collect();
        let mut ask: Vec<Order> = resp.asks.iter().map(|x| symbol.order(x.0, x.1)).collect();
        bid.retain(|x| x.price >= symbol.min_price);
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        ask.retain(|x| x.price <= symbol.max_price);
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        Ok(Depth { bid, ask, version })
    }
}
