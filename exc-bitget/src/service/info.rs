use super::Bitget;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tower::ServiceExt;

impl Bitget {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.next_update,
            interval: resp.funding_rate_interval * 60 * 60 * 1000,
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
