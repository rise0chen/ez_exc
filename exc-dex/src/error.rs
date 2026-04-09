use alloy::contract::Error;
use exc_util::error::ExchangeError;

pub fn map_err(e: Error) -> ExchangeError {
    ExchangeError::Other(e.into())
}
