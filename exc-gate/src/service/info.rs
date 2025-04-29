use super::Gate;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tower::ServiceExt;

impl Gate {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let info = if symbol.is_spot() {
            FundingRate::default()
        } else {
            use crate::futures_api::http::info::GetFundingRateRequest;
            let req = GetFundingRateRequest { contract: symbol_id };
            let resp = self.oneshot(req).await?;
            FundingRate {
                rate: resp.funding_rate,
                time: resp.funding_next_apply * 1000,
                interval: resp.funding_interval * 1000,
            }
        };
        Ok(info)
    }
}
