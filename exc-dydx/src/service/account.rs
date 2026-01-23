use super::Dydx;
use bigdecimal::ToPrimitive;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};

impl Dydx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        let account = self.wallet().account_offline(0)?;
        let subaccount = account.subaccount(0)?;
        let resp = self.indexer().accounts().get_subaccount(&subaccount).await?;
        Ok(resp.equity.to_f64().unwrap())
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let account = self.wallet().account_offline(0)?;
        let subaccount = account.subaccount(0)?;
        let resp = self.indexer().accounts().get_subaccount_perpetual_positions(&subaccount, None).await?;
        let resp = resp.iter().filter(|x| x.market == symbol_id);

        let (mut short_size, mut short_val) = (0.0, 0.0);
        let (mut long_size, mut long_val) = (0.0, 0.0);
        for x in resp {
            let size = x.size.to_f64().unwrap();
            let price = x.entry_price.to_f64().unwrap();
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
                price: if long_size ==0.0{0.0}else{long_val / long_size},
            },
            Position {
                size: short_size,
                price: if short_size ==0.0{0.0}else{short_val / short_size},
            },
        ))
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        self.get_positions(symbol).await.map(|(long, short)| {
            let size = long.size - short.size;
            let price = if (long.size + short.size)==0.0{0.0}else{(long.size * long.price + short.size * short.price) / (long.size + short.size)};
            Position { size, price }
        })
    }
}
