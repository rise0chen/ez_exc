use super::Paradex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};

fn order((p, s): (String, String)) -> Order {
    Order::new(p.parse().unwrap(), s.parse().unwrap())
}

impl Paradex {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol = crate::symnol::symbol_id(symbol);
        let opts = paradex::structs::OrderBookParams {
            depth: Some(limit),
            price_tick: None,
        };
        let resp = self.http.orderbook(symbol, opts).await.map_err(|e| ExchangeError::Other(e.into()))?;
        Ok(Depth {
            bid: resp.bids.into_iter().map(order).collect(),
            ask: resp.asks.into_iter().map(order).collect(),
            version: resp.last_updated_at,
        })
    }
}
