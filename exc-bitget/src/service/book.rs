use super::Bitget;
use exc_core::ExchangeError;
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
            Depth {
                bid: resp.b.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.a.iter().map(|x| Order::new(x.0, x.1)).collect(),
                version: resp.ts,
            }
        } else {
            use crate::api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                category: "USDT-FUTURES",
                symbol: symbol_id,
                limit,
            };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.b.iter().map(|x| Order::new(x.0, x.1)).collect(),
                ask: resp.a.iter().map(|x| Order::new(x.0, x.1)).collect(),
                version: resp.ts,
            }
        };
        Ok(bid_ask)
    }
}
