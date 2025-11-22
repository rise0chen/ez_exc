use super::Bybit;
use exc_core::ExchangeError;
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
        Ok(Depth {
            bid: resp.b.iter().map(|x| Order::new(x.0, x.1)).collect(),
            ask: resp.a.iter().map(|x| Order::new(x.0, x.1)).collect(),
            version: resp.ts,
        })
    }
}
