use super::Bitunix;
use crate::futures_api::types::OrderSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        if self.key.web_key.is_some() {
            use crate::futures_web::http::account::GetBalanceRequest;
            let req = GetBalanceRequest { coin: "USDT" };
            let resp = self.oneshot(req).await?;
            return Ok(Balance {
                spot: resp.spot_total,
                future: resp.futures_total,
                finance: resp.earn_total,
                total: resp.total,
            });
        }
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { margin_coin: "USDE".into() };
        let resp = self.oneshot(req).await?;
        let usde = resp.available + resp.margin + resp.cross_unrealized_p_n_l;
        let req = GetBalanceRequest { margin_coin: "USDT".into() };
        let resp = self.oneshot(req).await?;
        let usdt = resp.available + resp.margin + resp.cross_unrealized_p_n_l;
        Ok(Balance::new(0.0, usdt + 0.985 * usde, 0.0))
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
