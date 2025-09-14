use super::Gate;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::StRate;

impl Gate {
    pub async fn get_st_rate(&mut self, _symbol: &Symbol) -> Result<StRate, ExchangeError> {
        Err(ExchangeError::OrderNotFound)
    }
}
