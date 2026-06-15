use super::Xt;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Xt {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                symbol: symbol_id,
                level: limit,
            };
            let resp = self.oneshot(req).await?;
            let version = resp.t;
            let mut bid = if symbol.can_trade && (symbol.can_open || symbol.position > 0.0) {
                resp.b.iter().map(|x| symbol.order(x.0, x.1)).collect()
            } else {
                Vec::new()
            };
            let mut ask = if symbol.can_trade && (symbol.can_open || symbol.position < 0.0) {
                resp.a.iter().map(|x| symbol.order(x.0, x.1)).collect()
            } else {
                Vec::new()
            };
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
