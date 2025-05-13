use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tokio::sync::oneshot::Sender;

pub struct GetFundingRateRequest {
    pub symbol: Symbol,
    pub ch: Sender<Result<FundingRate, ExchangeError>>,
}
