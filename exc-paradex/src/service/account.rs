use super::Paradex;
use exc_util::error::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};

impl Paradex {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        let resp = self.http.account_information().await.map_err(|e| ExchangeError::Other(e.into()))?;
        Ok(resp.total_collateral)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.http.positions().await.map_err(|e| ExchangeError::Other(e.into()))?;
        let resp = resp.results.iter().filter(|x| x.market == symbol_id);

        let (mut short_size, mut short_val) = (0.0, 0.0);
        let (mut long_size, mut long_val) = (0.0, 0.0);
        for x in resp {
            let size = x.size;
            let price = x.average_entry_price_usd;
            if x.size < 0.0 {
                short_size -= size;
                short_val -= size * price;
            } else {
                long_size = size;
                long_val += size * price;
            }
        }
        Ok((
            Position {
                size: symbol.token_size(long_size),
                price: if long_size == 0.0 {
                    0.0
                } else {
                    symbol.token_price(long_val / long_size)
                },
            },
            Position {
                size: symbol.token_size(short_size),
                price: if short_size == 0.0 {
                    0.0
                } else {
                    symbol.token_price(short_val / short_size)
                },
            },
        ))
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
