use super::Lighter;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::StRate;

impl Lighter {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
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
