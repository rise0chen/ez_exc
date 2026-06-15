use super::Dydx;
use bigdecimal::ToPrimitive;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use time::OffsetDateTime;

impl Dydx {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer().markets().get_perpetual_market_orderbook(&symbol_id).await?;
        let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        let mut bid: Vec<Order> = resp
            .bids
            .iter()
            .map(|x| symbol.order(x.price.to_f64().unwrap(), x.size.to_f64().unwrap()))
            .collect();
        let mut ask: Vec<Order> = resp
            .asks
            .iter()
            .map(|x| symbol.order(x.price.to_f64().unwrap(), x.size.to_f64().unwrap()))
            .collect();
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        Ok(Depth { bid, ask, version })
    }
}
