use super::Bitget;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Bitget {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.eff_equity)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {};
            let resp = self.oneshot(req).await?;
            let size = resp
                .assets
                .iter()
                .find(|x| x.coin == symbol.base.as_str())
                .map(|x| x.balance)
                .unwrap_or(0.0);
            Ok((Position::new(size), Position::default()))
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                category: "USDT-FUTURES",
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?.list.unwrap_or_default();
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.pos_side == "short" {
                    short_size += x.total;
                    short_val += x.total * x.avg_price;
                } else {
                    long_size = x.total;
                    long_val += x.total * x.avg_price;
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
