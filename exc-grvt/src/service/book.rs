use super::Grvt;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use grvt_rust_sdk::types::BookRequest;

impl Grvt {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let req = BookRequest {
            instrument: symbol_id,
            depth: 10,
        };
        let resp = self.http.book_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?.result;
        Ok(Depth {
            bid: resp.bids.iter().map(|x| symbol.order(x.price, x.size)).collect(),
            ask: resp.asks.iter().map(|x| symbol.order(x.price, x.size)).collect(),
            version: (resp.event_time / 1_000_000) as u64,
        })
    }
}
