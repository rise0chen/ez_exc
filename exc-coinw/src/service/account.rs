use crate::futures_api::types::PositionSide;

use super::Coinw;
use exc_util::error::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Coinw {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::{GetAssetsRequest, GetBalanceRequest};
        let req = GetBalanceRequest {};
        let ballance = self.oneshot(req).await?;
        let req = GetAssetsRequest {};
        let assets = self.oneshot(req).await?;
        Ok(ballance.value + assets.al_freeze + assets.al_margin)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { instrument: symbol_id };
            let resp = self.oneshot(req).await?;
            let (mut short_id, mut short_size, mut short_val) = (String::new(), 0.0, 0.0);
            let (mut long_id, mut long_size, mut long_val) = (String::new(), 0.0, 0.0);
            for x in resp {
                match x.direction {
                    PositionSide::Short => {
                        short_id = x.id.into_string();
                        short_size += x.current_piece;
                        short_val += x.current_piece * x.open_price.unwrap_or(0.0);
                    }
                    PositionSide::Long => {
                        long_id = x.id.into_string();
                        long_size = x.current_piece;
                        long_val += x.current_piece * x.open_price.unwrap_or(0.0);
                    }
                    _ => {
                        if x.current_piece.is_sign_negative() {
                            short_id = x.id.into_string();
                            short_size += x.current_piece.abs();
                            short_val += x.current_piece * x.open_price.unwrap_or(0.0);
                        } else {
                            long_id = x.id.into_string();
                            long_size = x.current_piece;
                            long_val += x.current_piece * x.open_price.unwrap_or(0.0);
                        }
                    }
                }
            }
            Ok((
                Position {
                    id: long_id,
                    size: symbol.token_size(long_size),
                    price: if long_size == 0.0 {
                        0.0
                    } else {
                        symbol.token_price(long_val / long_size)
                    },
                },
                Position {
                    id: short_id,
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
