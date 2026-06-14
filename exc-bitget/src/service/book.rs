use super::Bitget;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Bitget {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            use crate::api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                category: "SPOT",
                symbol: symbol_id,
                limit,
            };
            let resp = self.oneshot(req).await?;
            let version = resp.ts;
            let mut bid: Vec<Order> = resp.b.iter().map(|x| symbol.order(x.0, x.1)).collect();
            let mut ask: Vec<Order> = resp.a.iter().map(|x| symbol.order(x.0, x.1)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        } else {
            use crate::api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                category: "USDT-FUTURES",
                symbol: symbol_id,
                limit,
            };
            let resp = self.oneshot(req).await?;
            let version = resp.ts;
            let mut bid: Vec<Order> = resp.b.iter().map(|x| symbol.order(x.0, x.1)).collect();
            let mut ask: Vec<Order> = resp.a.iter().map(|x| symbol.order(x.0, x.1)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
