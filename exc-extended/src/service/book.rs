use super::Extended;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use time::OffsetDateTime;
use tower::ServiceExt;

impl Extended {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::book::GetDepthRequest;
        let req = GetDepthRequest { market: symbol_id };
        let resp = self.oneshot(req).await?;
        let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        let mut bid = if symbol.can_trade && (symbol.can_open || symbol.position > 0.0) {
            resp.bid.iter().map(|x| symbol.order(x.price, x.qty)).collect()
        } else {
            Vec::new()
        };
        let mut ask = if symbol.can_trade && (symbol.can_open || symbol.position < 0.0) {
            resp.ask.iter().map(|x| symbol.order(x.price, x.qty)).collect()
        } else {
            Vec::new()
        };
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        Ok(Depth { bid, ask, version })
    }
}
