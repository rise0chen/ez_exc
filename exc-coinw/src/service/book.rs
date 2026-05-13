use super::Coinw;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use tower::ServiceExt;

impl Coinw {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { base: symbol_id };
            let resp = self.oneshot(req).await?;
            Depth {
                bid: resp.bids.iter().map(|x| symbol.order(x.p, symbol.contract_size(x.m).as_f64())).collect(),
                ask: resp.asks.iter().map(|x| symbol.order(x.p, symbol.contract_size(x.m).as_f64())).collect(),
                version: (resp.t) as u64,
            }
        };
        Ok(bid_ask)
    }
}
