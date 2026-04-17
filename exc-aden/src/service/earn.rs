use super::Aden;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::{common_st_rate, StRate};

impl Aden {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        if let Some(rate) = common_st_rate(symbol) {
            return Ok(rate);
        }
        let _coin: String = match symbol.base.as_str() {
            "" => {
                return Ok(StRate {
                    rate: 1.0,
                    start_time: 0,
                    apy: 0.0,
                })
            }
            _ => return Err(ExchangeError::OrderNotFound),
        };
    }
}
