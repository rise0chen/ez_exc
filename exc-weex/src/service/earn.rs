use super::Weex;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::StRate;

impl Weex {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        let _coin: String = match symbol.base.as_str() {
            "PAXG" => {
                return Ok(StRate {
                    rate: 1.01,
                    start_time: 0,
                    apy: 0.0,
                })
            }
            _ => return Err(ExchangeError::OrderNotFound),
        };
    }
}
