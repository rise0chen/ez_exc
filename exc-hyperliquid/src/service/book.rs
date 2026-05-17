use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;

impl Hyperliquid {
    pub async fn get_depth(&mut self, symbol: &Symbol, _limit: u16) -> Result<Depth, ExchangeError> {
        let coin = crate::symnol::symbol_id(symbol);
        if let Some(ch) = self.ws.books.get(&coin) {
            let mut book = ch.borrow().clone();
            if book.is_valid() {
                book.ask.iter_mut().for_each(|x| {
                    x.price = symbol.token_price(x.price);
                    x.size = symbol.token_size(x.size);
                });
                book.bid.iter_mut().for_each(|x| {
                    x.price = symbol.token_price(x.price);
                    x.size = symbol.token_size(x.size);
                });
                return Ok(book);
            }
        }
        Err(ExchangeError::OrderNotFound)
    }
}
