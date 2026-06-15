use super::Paradex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};

impl Paradex {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let opts = paradex::structs::OrderBookParams {
            depth: Some(limit),
            price_tick: None,
        };
        let (ask, bid, version) = if self.key.pro {
            let resp = self.http.orderbook(symbol_id, opts).await.map_err(|e| ExchangeError::Other(e.into()))?;
            (resp.asks, resp.bids, resp.last_updated_at)
        } else {
            let resp = self
                .http
                .orderbook_interactive(symbol_id, opts)
                .await
                .map_err(|e| ExchangeError::Other(e.into()))?;
            (resp.asks, resp.bids, resp.last_updated_at)
        };
        let mut bid: Vec<Order> = bid.iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s.parse().unwrap())).collect();
        let mut ask: Vec<Order> = ask.iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s.parse().unwrap())).collect();
        bid.sort_by(|a, b| b.price.total_cmp(&a.price));
        ask.sort_by(|a, b| a.price.total_cmp(&b.price));
        Ok(Depth { bid, ask, version })
    }
}
