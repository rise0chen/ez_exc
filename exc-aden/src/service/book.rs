use super::Aden;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Aden {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { contract: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            let version = (resp.update * 1000.0) as u64;
            let mut bid: Vec<Order> = resp.bids.iter().map(|x| symbol.order(x.p, x.s)).collect();
            let mut ask: Vec<Order> = resp.asks.iter().map(|x| symbol.order(x.p, x.s)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
