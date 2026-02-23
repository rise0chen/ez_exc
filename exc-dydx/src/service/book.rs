use super::Dydx;
use bigdecimal::ToPrimitive;
use dydx::indexer::types::OrderbookResponsePriceLevel;
use dydx::indexer::ListPerpetualMarketsOpts;
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
        let opts = Some(ListPerpetualMarketsOpts {
            ticker: Some(symbol_id.clone()),
            limit: None,
        });
        let mut markets = self.indexer().markets().get_perpetual_markets(opts).await?;
        let price = markets.remove(&symbol_id).unwrap().oracle_price;
        let price = price.map(|x| x.to_f64().unwrap()).unwrap_or_default();
        let bid_order = [Order::new(price * 0.999, 50.0 / price)];
        let ask_order = [Order::new(price * 1.001, 50.0 / price)];

        Ok(Depth {
            bid: bid_order.into_iter().chain(resp.bids.iter().map(order)).collect(),
            ask: ask_order.into_iter().chain(resp.asks.iter().map(order)).collect(),
            version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
        })
    }
}
