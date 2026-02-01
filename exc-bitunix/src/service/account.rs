use super::Bitunix;
use crate::futures_api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { margin_coin: "USDE".into() };
        let resp = self.oneshot(req).await?;
        let usde = resp.available + resp.margin + resp.cross_unrealized_p_n_l;
        let req = GetBalanceRequest { margin_coin: "USDT".into() };
        let resp = self.oneshot(req).await?;
        let usdt = resp.available + resp.margin + resp.cross_unrealized_p_n_l;
        Ok(usdt + 0.985 * usde)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.side == OrderSide::Sell {
                    short_size += x.qty;
                    short_val += x.qty * x.avg_open_price;
                } else {
                    long_size = x.qty;
                    long_val += x.qty * x.avg_open_price;
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
