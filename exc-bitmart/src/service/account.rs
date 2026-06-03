use super::Bitmart;
use crate::futures_api::types::PositionSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Bitmart {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        use crate::futures_api::http::account::{GetBalanceRequest, GetEarnRequest};
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        let resp = resp.iter().find(|x| x.currency == "USDT");
        let Some(resp) = resp else { return Err(ExchangeError::OrderNotFound) };
        let req = GetEarnRequest {};
        let earn = self.oneshot(req).await?;
        Ok(Balance::new(0.0, resp.equity, earn.total_user_sum))
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
                match x.position_side {
                    PositionSide::Short => {
                        short_size += x.position_amount;
                        short_val += x.position_value;
                    }
                    PositionSide::Long => {
                        long_size = x.position_amount;
                        long_val += x.position_value;
                    }
                    _ => {
                        if x.position_amount.is_sign_negative() {
                            short_size += x.position_amount.abs();
                            short_val += x.position_value;
                        } else {
                            long_size = x.position_amount;
                            long_val += x.position_value;
                        }
                    }
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
