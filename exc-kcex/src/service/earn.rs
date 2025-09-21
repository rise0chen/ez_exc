use super::Kcex;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::StRate;

impl Kcex {
    pub async fn get_st_rate(&mut self, _symbol: &Symbol) -> Result<StRate, ExchangeError> {
        Err(ExchangeError::OrderNotFound)
    }
}
