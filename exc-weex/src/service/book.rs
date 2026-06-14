use super::Weex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use time::OffsetDateTime;
use tower::ServiceExt;

impl Weex {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                symbol: symbol_id,
                limit: 15,
            };
            let resp = self.oneshot(req).await?;
            let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
            let mut bid: Vec<Order> = resp.bids.iter().map(|x| symbol.order(x.0, x.1)).collect();
            let mut ask: Vec<Order> = resp.asks.iter().map(|x| symbol.order(x.0, x.1)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
