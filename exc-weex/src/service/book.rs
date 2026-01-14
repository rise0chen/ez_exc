use super::Weex;
use exc_core::ExchangeError;
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
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
            }
        };
        Ok(bid_ask)
    }
}
