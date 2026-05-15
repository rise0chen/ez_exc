use super::Toobit;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use tower::ServiceExt;

impl Toobit {
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
                bid: resp.b.iter().map(|x| symbol.order(x.0, x.1)).collect(),
                ask: resp.a.iter().map(|x| symbol.order(x.0, x.1)).collect(),
                version: resp.t,
            }
        };
        Ok(bid_ask)
    }
}
