use super::Lighter;
use exc_util::error::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Lighter {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetAccountRequest;
        let req = GetAccountRequest {
            by: "index",
            value: self.key.account_index,
        };
        let resp = self.oneshot(req).await?.accounts.pop();
        resp.map(|resp| resp.cross_asset_value).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::account::GetAccountRequest;
            let req = GetAccountRequest {
                by: "index",
                value: self.key.account_index,
            };
            let resp = self.oneshot(req).await?.accounts.pop();
            let positions = resp.map(|resp| resp.positions).unwrap_or(Vec::new());
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &positions {
                if x.market_id != symbol_id {
                    continue;
                }
                if x.sign.is_negative() {
                    short_size += x.position;
                    short_val += x.position * x.avg_entry_price;
                } else {
                    long_size = x.position;
                    long_val += x.position * x.avg_entry_price;
                }
            }
            Ok((
                Position {
                    size: long_size,
                    price: if long_size == 0.0 { 0.0 } else { long_val / long_size },
                },
                Position {
                    size: short_size,
                    price: if short_size == 0.0 { 0.0 } else { short_val / short_size },
                },
            ))
        }
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        self.get_positions(symbol).await.map(|(long, short)| {
            let size = long.size - short.size;
            let price = if (long.size + short.size) == 0.0 {
                0.0
            } else {
                (long.size * long.price + short.size * short.price) / (long.size + short.size)
            };
            Position { size, price }
        })
    }
}
