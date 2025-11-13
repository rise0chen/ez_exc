use super::Bitunix;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use time::OffsetDateTime;
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { symbol: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                price: (resp.asks[0].0 + resp.bids[0].0) / 2.0,
                version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
            }
        };
        Ok(bid_ask)
    }
}
