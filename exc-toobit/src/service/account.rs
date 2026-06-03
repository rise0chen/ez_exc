use super::Toobit;
use crate::futures_api::types::PositionSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Toobit {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        use crate::futures_web::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { unit: "USDT" };
        let resp = self.oneshot(req).await?;
        Ok(Balance {
            spot: resp.coin_asset,
            future: resp.futures_coin_asset,
            finance: resp.earn_asset,
            total: resp.total_asset,
        })
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if x.side == PositionSide::Short {
                    short_size += x.position;
                    short_val += x.avg_price;
                } else {
                    long_size = x.position;
                    long_val += x.avg_price;
                }
            }
            Ok((
                Position {
                    id: String::new(),
                    size: symbol.token_size(long_size),
                    price: if long_size == 0.0 {
                        0.0
                    } else {
                        symbol.token_price(long_val / long_size)
                    },
                },
                Position {
                    id: String::new(),
                    size: symbol.token_size(short_size),
                    price: if short_size == 0.0 {
                        0.0
                    } else {
                        symbol.token_price(short_val / short_size)
                    },
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
