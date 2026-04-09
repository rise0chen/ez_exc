use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;

impl Hyperliquid {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let coin = crate::symnol::symbol_id(symbol);
        if let Some(ch) = self.ws.books.get(&coin) {
            Ok(ch.borrow().clone())
        } else {
            Err(ExchangeError::OrderNotFound)
        }
    }
}
