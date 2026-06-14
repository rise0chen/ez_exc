use super::Gate;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};
use tower::ServiceExt;

impl Gate {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            if let Some(ch) = self.ws_spot.books.get(&symbol_id) {
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
                    book.bid.retain(|x| x.price >= symbol.min_price);
                    book.bid.sort_by(|a, b| b.price.total_cmp(&a.price));
                    book.ask.retain(|x| x.price <= symbol.max_price);
                    book.ask.sort_by(|a, b| a.price.total_cmp(&b.price));
                    return Ok(book);
                }
            }
            if !self.ws_spot.symbols.is_empty() {
                tracing::warn!("gate get depth spot:{} by http", symbol_id);
            }
            use crate::spot_api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                currency_pair: symbol_id,
                limit,
            };
            let resp = self.oneshot(req).await?;
            let version = resp.update;
            let mut bid: Vec<Order> = resp.bids.iter().map(|x| symbol.order(x.0, x.1)).collect();
            let mut ask: Vec<Order> = resp.asks.iter().map(|x| symbol.order(x.0, x.1)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        } else {
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest { contract: symbol_id, limit };
            let resp = self.oneshot(req).await?;
            let version = (resp.update * 1000.0) as u64;
            let mut bid: Vec<Order> = resp.bids.iter().map(|x| symbol.order(x.p, x.s)).collect();
            let mut ask: Vec<Order> = resp.asks.iter().map(|x| symbol.order(x.p, x.s)).collect();
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
