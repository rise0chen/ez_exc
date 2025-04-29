use super::Okx;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tower::ServiceExt;

impl Okx {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { inst_id: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.funding_time,
            interval: resp.next_funding_time - resp.funding_time,
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
