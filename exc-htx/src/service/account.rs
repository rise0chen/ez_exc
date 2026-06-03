use super::Htx;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::account::{Balance, Position};
use tower::ServiceExt;

impl Htx {
    pub async fn get_balance(&mut self) -> Result<Balance, ExchangeError> {
        let req = crate::spot_api::http::account::GetBalanceRequest {
            account_type: "spot",
            valuation_currency: "USD",
        };
        let spot = self.oneshot(req).await?;
        let req = crate::futures_api::http::account::GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        let req = crate::spot_api::http::account::GetEarnRequest { page_num: 1, page_size: 10 };
        let earn = self.oneshot(req).await?;
        let finance = earn
            .items
            .iter()
            .map(|x| if x.currency.contains("USD") { x.total_amount } else { 0.0 })
            .sum();
        Ok(Balance::new(spot.balance, resp.equity, finance))
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            use crate::spot_api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                account_id: self.key.account_id,
            };
            let resp = self.oneshot(req).await?.data.list;
            let size = resp.iter().filter(|x| x.currency == symbol.base.to_lowercase()).map(|x| x.balance).sum();
            Ok((Position::new(size), Position::default()))
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            use crate::futures_api::types::OrderSide;
            let req = GetPositionRequest { contract_code: symbol_id };
            let resp = self.oneshot(req).await?;
            let (mut short_size, mut short_val) = (0.0, 0.0);
            let (mut long_size, mut long_val) = (0.0, 0.0);
            for x in &resp {
                if matches!(x.direction, OrderSide::Sell) {
                    short_size += x.volume;
                    short_val += x.volume * x.open_avg_price;
                } else {
                    long_size = x.volume;
                    long_val += x.volume * x.open_avg_price;
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
