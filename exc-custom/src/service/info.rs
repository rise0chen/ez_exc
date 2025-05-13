use super::Custom;
use crate::api::info::GetFundingRateRequest;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tokio::sync::oneshot;
use tower::ServiceExt;

impl Custom {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = GetFundingRateRequest {
            symbol: symbol.clone(),
            ch: tx,
        };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
}
