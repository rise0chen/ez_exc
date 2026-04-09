use super::Custom;
use crate::api::info::{GetFundingRateHistoryRequest, GetFundingRateRequest};
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tokio::sync::oneshot;
use tower::ServiceExt;

impl Custom {
    pub async fn get_index_price(&mut self, _symbol: &Symbol) -> Result<f64, ExchangeError> {
        Ok(0.0)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = GetFundingRateRequest {
            symbol: symbol.clone(),
            ch: tx,
        };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = GetFundingRateHistoryRequest {
            symbol: symbol.clone(),
            day,
            ch: tx,
        };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
}
