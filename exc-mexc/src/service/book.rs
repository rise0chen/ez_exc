use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::BidAsk;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_bid_ask(&mut self, symbol: &Symbol) -> Result<BidAsk, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            use crate::spot_api::http::book::GetBidAskRequest;
            let req = GetBidAskRequest { symbol: symbol_id, limit: 1 };
            let resp = self.oneshot(req).await?;
            BidAsk {
                bid: (resp.bids[0].0, resp.bids[0].1),
                ask: (resp.asks[0].0, resp.asks[0].1),
                version: resp.last_update_id,
            }
        } else {
            use crate::futures_api::http::book::GetBidAskRequest;
            let req = GetBidAskRequest { symbol: symbol_id, limit: 1 };
            let resp = self.oneshot(req).await?;
            BidAsk {
                bid: (resp.bids[0].0, resp.bids[0].1),
                ask: (resp.asks[0].0, resp.asks[0].1),
                version: resp.version,
            }
        };
        Ok(bid_ask)
    }
}
