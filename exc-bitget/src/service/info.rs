use super::Bitget;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
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

    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        if symbol.is_spot() {
            return Ok(Vec::new());
        }
        if day > 5 {
            return Err(ExchangeError::Forbidden(anyhow::anyhow!("day max 5")));
        }
        let symbol_id = crate::symnol::symbol_id(symbol);

        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        use crate::api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            category: "USDT-FUTURES",
            symbol: symbol_id,
            limit: day * 24,
        };
        let mut resp = self.oneshot(req).await?.result_list;
        resp.retain(|x| x.funding_rate_timestamp > start_time);
        let interval = (day as u64 * 24 * 60 * 60 * 1000) / resp.len() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.funding_rate_timestamp,
                interval,
            })
            .collect())
    }
}
