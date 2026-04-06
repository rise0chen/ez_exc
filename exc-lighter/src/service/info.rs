use super::Lighter;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Lighter {
    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let Some(resp) = self.ws.get_market(&symbol.base_id).await else {
            return Err(ExchangeError::OrderNotFound);
        };
        Ok(resp.index_price.parse::<f64>().unwrap())
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let Some(resp) = self.ws.get_market(&symbol.base_id).await else {
            return Err(ExchangeError::OrderNotFound);
        };
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.current_funding_rate.parse::<f64>().unwrap() / 100.0,
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
        use crate::futures_api::http::info::GetFundingRateHistoryRequest;
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        let req = GetFundingRateHistoryRequest {
            market_id: symbol_id,
            resolution: "1h",
            start_timestamp: start_time,
            end_timestamp: 5000000000000,
            count_back: day * 24,
        };
        let mut resp = self.oneshot(req).await?.fundings;
        if resp.is_empty() {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = (day as u64 * 24 * 60 * 60 * 1000) / resp.len() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.rate / 100.0,
                time: x.timestamp * 1000,
                interval,
            })
            .collect())
    }
}
