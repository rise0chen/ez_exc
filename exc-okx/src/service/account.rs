use super::Okx;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Okx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { ccy: Some("USDT".into()) };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.adj_eq).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                ccy: Some(symbol.base.as_str().into()),
            };
            let resp = self.oneshot(req).await?.pop().map(|x| x.details).unwrap_or_default();
            let size = resp.iter().find(|x| x.ccy == symbol.base.as_str()).map(|x| x.avail_bal).unwrap_or(0.0);
            Ok((Position::new(size), Position::default()))
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest { inst_id: symbol_id };
            let resp = self.oneshot(req).await?;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.pos_side == "short" {
                    short_size += x.pos;
                    short_val += x.pos * x.open_avg_px;
                } else {
                    long_size = x.pos;
                    long_val += x.pos * x.open_avg_px;
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
