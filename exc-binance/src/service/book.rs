use super::Binance;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Binance {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            use crate::spot_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { symbol: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                price: (resp.asks[0].0 + resp.bids[0].0) / 2.0,
                version: resp.last_update_id,
            }
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { symbol: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                price: (resp.asks[0].0 + resp.bids[0].0) / 2.0,
                version: resp.last_update_id,
            }
        };
        Ok(bid_ask)
    }
}
