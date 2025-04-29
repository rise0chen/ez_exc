use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let info = if symbol.is_spot() {
            FundingRate::default()
        } else {
            use crate::futures_api::http::info::GetFundingRateRequest;
            let req = GetFundingRateRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?;
            FundingRate {
                rate: resp.funding_rate,
                time: resp.next_settle_time,
                interval: resp.collect_cycle * 60 * 60 * 1000,
            }
        };
        Ok(info)
    }
}
