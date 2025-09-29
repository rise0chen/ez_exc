use alloy::contract::Error;
use exc_core::ExchangeError;

pub fn map_err(e: Error) -> ExchangeError {
    ExchangeError::Other(e.into())
}
