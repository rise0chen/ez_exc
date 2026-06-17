use super::Lighter;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use time::OffsetDateTime;

impl Lighter {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let Some(resp) = self.ws.get_order_book(&symbol.base_id).await else {
            return Ok(Depth::default());
        };
        let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        let mut bid: Vec<_> = resp
            .bids
            .into_iter()
            .map(|x| symbol.order(x.price.parse().unwrap(), x.size.parse().unwrap()))
            .collect();
        let mut ask: Vec<_> = resp
            .asks
            .into_iter()
            .map(|x| symbol.order(x.price.parse().unwrap(), x.size.parse().unwrap()))
            .collect();
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        bid.truncate(limit as usize);
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        ask.truncate(limit as usize);
        let bid_ask = Depth { bid, ask, version };
        Ok(bid_ask)
    }
}
