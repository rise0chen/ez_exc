use super::Dydx;
use bigdecimal::ToPrimitive;
use dydx::indexer::types::OrderbookResponsePriceLevel;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use time::OffsetDateTime;

fn order(x: &OrderbookResponsePriceLevel) -> Order {
    Order::new(x.price.to_f64().unwrap(), x.size.to_f64().unwrap())
}

impl Dydx {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer().markets().get_perpetual_market_orderbook(&symbol_id).await?;

        Ok(Depth {
            bid: resp.bids.iter().map(order).collect(),
            ask: resp.asks.iter().map(order).collect(),
            version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
        })
    }
}
