use super::Dydx;
use bigdecimal::ToPrimitive;
use chrono::{Duration, Utc};
use dydx::indexer::GetHistoricalFundingOpts;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;

impl Dydx {
    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer().markets().get_perpetual_market(&symbol_id).await?;
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.next_funding_rate.to_f64().unwrap(),
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let start_time = Utc::now() - Duration::days(day as i64);
        let mut resp = self
            .indexer()
            .markets()
            .get_perpetual_market_historical_funding(
                &symbol_id,
                Some(GetHistoricalFundingOpts {
                    limit: Some(day as u32 * 24),
                    effective_before_or_at: None,
                    effective_before_or_at_height: None,
                }),
            )
            .await?;
        resp.retain(|x| x.effective_at > start_time);
        let interval = (day as u64 * 24 * 60 * 60 * 1000) / resp.len() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.rate.to_f64().unwrap(),
                time: x.effective_at.timestamp_millis() as u64,
                interval,
            })
            .collect())
    }
}
