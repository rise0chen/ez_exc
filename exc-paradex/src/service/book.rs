use super::Paradex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;

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
        Ok(Depth {
            bid: bid.iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s.parse().unwrap())).collect(),
            ask: ask.iter().map(|(p, s)| symbol.order(p.parse().unwrap(), s.parse().unwrap())).collect(),
            version,
        })
    }
}
