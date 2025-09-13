use super::Gate;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;

impl Gate {
    pub async fn get_st_rate(&mut self, _symbol: &Symbol) -> Result<f64, ExchangeError> {
        Err(ExchangeError::OrderNotFound)
    }
}
