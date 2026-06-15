use super::Bybit;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Bybit {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::book::GetDepthRequest;
        let req = GetDepthRequest {
            category: symbol.kind,
            symbol: symbol_id,
            limit,
        };
        let resp = self.oneshot(req).await?;
        let version = resp.ts;
        let mut bid: Vec<Order> = resp.b.iter().map(|x| symbol.order(x.0, x.1)).collect();
        let mut ask: Vec<Order> = resp.a.iter().map(|x| symbol.order(x.0, x.1)).collect();
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        Ok(Depth { bid, ask, version })
    }
}
