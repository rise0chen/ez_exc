use super::Lbank;
use crate::futures_web::types::OrderSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use time::OffsetDateTime;
use tower::ServiceExt;

impl Lbank {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let bid_ask = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_web::http::book::GetDepthRequest;
            let req = GetDepthRequest {
                exchange_i_d: "Exchange",
                product_group: "SwapU",
                instrument_i_d: symbol_id,
                depth: limit,
            };
            let resp = self.oneshot(req).await?;
            let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
            let mut bid = Vec::new();
            let mut ask = Vec::new();
            for x in resp {
                let x = x.data;
                if OrderSide::from(x.direction).is_buy() {
                    bid.push(symbol.order(x.price, x.volume));
                } else {
                    ask.push(symbol.order(x.price, x.volume));
                }
            }
            bid.retain(|x| x.price >= symbol.min_price);
            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
            ask.retain(|x| x.price <= symbol.max_price);
            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
            Depth { bid, ask, version }
        };
        Ok(bid_ask)
    }
}
