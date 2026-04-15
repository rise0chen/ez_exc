use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};

impl Hyperliquid {
    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let coin = crate::symnol::symbol_id(symbol);
        if let Some(ch) = self.ws.index_prices.get(&coin) {
            Ok(*ch.borrow())
        } else {
            Err(ExchangeError::OrderNotFound)
        }
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let coin = crate::symnol::symbol_id(symbol);
        let rate = if let Some(ch) = self.ws.funding_rates.get(&coin) {
            *ch.borrow()
        } else {
            return Err(ExchangeError::OrderNotFound);
        };
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate,
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        if symbol.is_spot() {
            return Ok(Vec::new());
        }
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        let coin = crate::symnol::symbol_id(symbol);
        let mut resp = self.http.funding_history(coin, start_time, None).await?;
        resp.retain(|x| x.time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = resp[0].time - resp[1].time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate.as_f64(),
                time: x.time,
                interval,
            })
            .collect())
    }
}
