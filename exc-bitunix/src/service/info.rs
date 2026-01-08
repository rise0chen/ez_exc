use super::Bitunix;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_web::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        let interval = 24 * 60 * 60 * 1000 / resp.funding_times.len() as u64;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.funding_rate_next,
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }

    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        if symbol.is_spot() {
            return Ok(Vec::new());
        }
        if day > 5 {
            return Err(ExchangeError::Forbidden(anyhow::anyhow!("day max 5")));
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        let start_time = OffsetDateTime::now_utc() - Duration::days(day as i64);
        use crate::futures_web::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            symbol: symbol_id,
            start_time,
        };
        let resp = self.oneshot(req).await?;
        let interval = (day as u64 * 24 * 60 * 60 * 1000) / resp.len() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.settle_funding_rate,
                time: (x.settle_time.unix_timestamp_nanos() / 1_000_000) as u64,
                interval,
            })
            .collect())
    }
}
