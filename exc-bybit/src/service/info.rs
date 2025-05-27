use super::Bybit;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tower::ServiceExt;

impl Bybit {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest {
            category: symbol.kind,
            symbol: symbol_id,
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.next_funding_time,
            interval: 8 * 60 * 60 * 1000,
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
