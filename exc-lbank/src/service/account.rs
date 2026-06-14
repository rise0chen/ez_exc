use super::Lbank;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Lbank {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        use crate::futures_web::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { __auto_login: true };
        let ballance = self.oneshot(req).await?;
        Ok(Balance::new(
            ballance.spot_asset.to_usd,
            ballance.futures_asset_covert.to_usd,
            ballance.investment_asset.to_usd,
        ))
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_web::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                product_group: "SwapU",
                instrument: symbol_id,
            };
            let resp = self.oneshot(req).await?.data;
            let (mut short_id, mut short_size, mut short_val) = (String::new(), 0.0, 0.0);
            let (mut long_id, mut long_size, mut long_val) = (String::new(), 0.0, 0.0);
            for x in resp {
                if x.position < 0.0 {
                    short_id = x.position_i_d;
                    short_size += -x.position;
                    short_val += -x.position_cost;
                } else {
                    long_id = x.position_i_d;
                    long_size = x.position;
                    long_val += x.position_cost;
                }
            }
            Ok((
                Position {
                    id: long_id,
                    size: long_size,
                    price: if long_size == 0.0 { 0.0 } else { long_val / long_size },
                },
                Position {
                    id: short_id,
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
