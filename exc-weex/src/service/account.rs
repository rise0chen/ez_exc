use super::Weex;
use crate::futures_api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Weex {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.equity).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.side == OrderSide::Short {
                    short_size += x.size;
                    short_val += x.open_value;
                } else {
                    long_size = x.size;
                    long_val += x.open_value;
                }
            }
            Ok((
                Position {
                    size: long_size,
                    price: long_val / long_size,
                },
                Position {
                    size: short_size,
                    price: short_val / short_size,
                },
            ))
        }
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        self.get_positions(symbol).await.map(|(long, short)| {
            let size = long.size - short.size;
            let price = (long.size * long.price + short.size * short.price) / (long.size + short.size);
            Position { size, price }
        })
    }
}
