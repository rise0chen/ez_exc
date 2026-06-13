use super::Bitmex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Bitmex {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { currency: "MAMUSd" };
        let ballance = self.oneshot(req).await?;
        Ok(Balance::new(0.0, ballance.margin_balance as f64 / 1e6, 0.0))
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                filter: [("symbol".into(), symbol_id)].into(),
            };
            let resp = self.oneshot(req).await?;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in resp {
                if x.home_notional < 0.0 {
                    short_size += -x.home_notional;
                    short_val += -x.home_notional * x.avg_entry_price.unwrap_or(0.0);
                } else {
                    long_size = x.home_notional;
                    long_val += x.home_notional * x.avg_entry_price.unwrap_or(0.0);
                }
            }
            Ok((
                Position {
                    id: String::new(),
                    size: long_size,
                    price: if long_size == 0.0 { 0.0 } else { long_val / long_size },
                },
                Position {
                    id: String::new(),
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
            Position {
                id: String::new(),
                size,
                price,
            }
        })
    }
}
