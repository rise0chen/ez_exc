use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct GetFundingRateRequest {
    pub symbol: Symbol,
    pub ch: Sender<Result<FundingRate, ExchangeError>>,
}

#[derive(Debug)]
pub struct GetFundingRateHistoryRequest {
    pub symbol: Symbol,
    pub day: u8,
    pub ch: Sender<Result<Vec<FundingRate>, ExchangeError>>,
}
