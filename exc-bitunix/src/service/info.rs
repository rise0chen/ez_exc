use super::Bitunix;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Bitunix {
    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        if let Some(ch) = self.ws.index_prices.get(&symbol_id) {
            Ok(*ch.borrow())
        } else {
            Err(ExchangeError::OrderNotFound)
        }
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        let interval = resp.funding_interval * 60 * 60 * 1000;
        Ok(FundingRate {
            rate: resp.funding_rate / 100.0,
            time: resp.next_funding_time,
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
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        use crate::futures_api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            symbol: symbol_id,
            start_time,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.funding_time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].funding_time - resp[1].funding_time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.funding_time,
                interval,
            })
            .collect())
    }
}
