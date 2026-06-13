use super::Bitmex;
use crate::futures_api::types::OrderSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use time::OffsetDateTime;
use tower::ServiceExt;

impl Bitmex {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            if let Some(ch) = self.ws.books.get(&symbol_id) {
                let mut book = ch.borrow().clone();
                if book.is_valid() {
                    book.ask.iter_mut().for_each(|x| {
                        x.price = symbol.token_price(x.price);
                        x.size = symbol.token_size(x.size / symbol.multi_size);
                    });
                    book.bid.iter_mut().for_each(|x| {
                        x.price = symbol.token_price(x.price);
                        x.size = symbol.token_size(x.size / symbol.multi_size);
                    });
                    return Ok(book);
                }
            }
            tracing::warn!("bitmex get depth {} by http", symbol_id);
            use crate::futures_api::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                symbol: symbol_id,
                depth: limit,
            };
            let resp = self.oneshot(req).await?;
            let version = resp.iter().map(|x| x.transact_time.unix_timestamp_nanos()).max();
            let version = (version.unwrap_or(OffsetDateTime::now_utc().unix_timestamp_nanos()) / 1_000_000) as u64;
            let mut bid = Vec::new();
            let mut ask = Vec::new();
            for x in resp {
                if matches!(x.side, OrderSide::Buy) {
                    bid.push(symbol.order(x.price, x.size as f64));
                } else {
                    ask.push(symbol.order(x.price, x.size as f64));
                }
            }
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
