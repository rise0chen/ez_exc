use super::Bitunix;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let info = if symbol.is_spot() {
            FundingRate::default()
        } else {
            use crate::futures_web::http::info::GetFundingRateRequest;
            let req = GetFundingRateRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?;
            let interval = 24 * 60 * 60 * 1000 / resp.funding_times.len() as u64;
            let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
            FundingRate {
                rate: resp.funding_rate_next,
                time: ((now / interval) + 1) * interval,
                interval,
            }
        };
        Ok(info)
    }
}
