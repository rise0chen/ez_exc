use super::Lighter;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use std::collections::BTreeMap;
use time::OffsetDateTime;

impl Lighter {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let Some(resp) = self.ws.get_order_book(&symbol.base_id).await else {
            return Ok(Depth::default());
        };
        let mut bids = BTreeMap::new();
        let mut asks = BTreeMap::new();
        resp.bids.into_iter().for_each(|x| {
            bids.entry(x.price)
                .and_modify(|curr| *curr += x.size.parse::<f64>().unwrap())
                .or_insert(x.size.parse::<f64>().unwrap());
        });
        resp.asks.into_iter().for_each(|x| {
            asks.entry(x.price)
                .and_modify(|curr| *curr += x.size.parse::<f64>().unwrap())
                .or_insert(x.size.parse::<f64>().unwrap());
        });
        let mut bid: Vec<_> = bids.into_iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s)).collect();
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        bid.truncate(limit as usize);
        let mut ask: Vec<_> = asks.into_iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s)).collect();
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        ask.truncate(limit as usize);
        let bid_ask = Depth {
            bid,
            ask,
            version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
        };
        Ok(bid_ask)
    }
}
