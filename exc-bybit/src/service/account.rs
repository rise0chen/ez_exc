use crate::api::types::OrderSide;

use super::Bybit;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Bybit {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {
            account_type: "UNIFIED",
            coin: Some("USDT".to_string()),
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| resp.total_margin_balance).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                account_type: "UNIFIED",
                coin: None,
            };
            let resp = self.oneshot(req).await?.list.pop().map(|x| x.coin).unwrap_or_default();
            let size = resp.iter().find(|x| x.coin == symbol.base.as_str()).map(|x| x.equity).unwrap_or(0.0);
            Ok((Position::new(size), Position::default()))
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                category: symbol.kind,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?.list;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.side == OrderSide::Sell {
                    short_size += x.size;
                    short_val += x.size * x.avg_price;
                } else {
                    long_size = x.size;
                    long_val += x.size * x.avg_price;
                }
            }
            Ok((
                Position {
                    size: long_size,
                    price: if long_size ==0.0{0.0}else{long_val / long_size},
                },
                Position {
                    size: short_size,
                    price: if short_size ==0.0{0.0}else{short_val / short_size},
                },
            ))
        }
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        self.get_positions(symbol).await.map(|(long, short)| {
            let size = long.size - short.size;
            let price = if (long.size + short.size)==0.0{0.0}else{(long.size * long.price + short.size * short.price) / (long.size + short.size)};
            Position { size, price }
        })
    }
}
