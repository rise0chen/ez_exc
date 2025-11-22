use super::Gate;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Gate {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            use crate::spot_api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                currency_pair: symbol_id,
                limit,
            };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                version: resp.update,
            }
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { contract: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| Order::new(x.p, x.s)).collect(),
                ask: resp.asks.iter().map(|x| Order::new(x.p, x.s)).collect(),
                version: (resp.update * 1000.0) as u64,
            }
        };
        Ok(bid_ask)
    }
}
