use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};

impl Hyperliquid {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        let resp = self.http.user_balances(self.key.user.parse().unwrap()).await?;
        Ok(resp
            .iter()
            .map(|x| if x.coin.contains("USD") { x.total } else { x.entry_ntl }.as_f64())
            .sum())
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        if symbol.is_spot() {
            let resp = self.http.user_balances(self.key.user.parse().unwrap()).await?;
            let resp = resp.iter().find(|x| x.coin == *symbol.base).ok_or(ExchangeError::OrderNotFound)?;
            return Ok((Position::new(resp.total.as_f64()), Position::new(0.0)));
        }
        let coin = crate::symnol::symbol_id(symbol);
        let dex = match coin.split_once(':') {
            Some((a, b)) => {
                if b.is_empty() {
                    None
                } else {
                    Some(a.to_string())
                }
            }
            None => None,
        };
        let resp = self.http.clearinghouse_state(self.key.user.parse().unwrap(), dex).await?;
        let resp = resp.asset_positions.iter().filter(|x| x.position.coin == coin);

        let (mut short_size, mut short_val) = (0.0, 0.0);
        let (mut long_size, mut long_val) = (0.0, 0.0);
        for x in resp {
            let x = &x.position;
            let size = x.szi.as_f64();
            let price = x.entry_px.unwrap_or_default().as_f64();
            if size < 0.0 {
                short_size += -size;
                short_val += -size * price;
            } else {
                long_size = size;
                long_val += size * price;
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
