use super::Extended;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Extended {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(Balance::new(0.0, resp.equity, 0.0))
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            use crate::futures_api::http::account::GetSpotBalanceRequest;
            let req = GetSpotBalanceRequest {};
            let resp = self.oneshot(req).await?;
            let Some(resp) = resp.iter().find(|x| symbol.base == x.asset) else {
                return Err(ExchangeError::OrderNotFound);
            };
            Ok((
                Position {
                    id: String::new(),
                    size: symbol.token_size(resp.balance),
                    price: symbol.token_price(resp.average_entry_price.unwrap_or_default()),
                },
                Position::new(0.0),
            ))
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { market: symbol_id.clone() };
            let resp = self.oneshot(req).await?;
            let (mut short_id, mut short_size, mut short_val) = (String::new(), 0.0, 0.0);
            let (mut long_id, mut long_size, mut long_val) = (String::new(), 0.0, 0.0);
            for x in &resp {
                if x.side.is_sell() {
                    short_id = x.id.to_string();
                    short_size += x.size;
                    short_val += x.size * x.open_price;
                } else {
                    long_id = x.id.to_string();
                    long_size = x.size;
                    long_val += x.size * x.open_price;
                }
            }
            Ok((
                Position {
                    id: short_id,
                    size: symbol.token_size(long_size),
                    price: if long_size == 0.0 {
                        0.0
                    } else {
                        symbol.token_price(long_val / long_size)
                    },
                },
                Position {
                    id: long_id,
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
