use super::Coinw;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime, UtcOffset};
use tower::ServiceExt;

impl Coinw {
    #[allow(unused_assignments)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = symbol.parse_prefix();
        let mut multi_size = symbol.multi_size;
        let mut precision_size = symbol.precision;
        let mut precision_price = symbol.precision_price;
        let mut min_size = symbol.min_size;
        let mut min_usd = symbol.min_usd;
        let mut fee = symbol.fee;

        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            multi_price = 1.0;
            multi_size = 1.0;
            fee = 0.0006;
        } else {
            use crate::futures_api::http::info::GetInfoRequest;
            let req = GetInfoRequest { name: symbol_id };
            let Some(a) = self.oneshot(req).await?.pop() else {
                return Err(ExchangeError::OrderNotFound);
            };
            multi_size = a.one_lot_size;
            precision_size = 0;
            precision_price = a.price_precision;
            min_size = a.min_size;
            min_usd = 0.0;
            fee = a.taker_fee;
        }
        if symbol.multi_price != multi_price {
            tracing::error!("coinw multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("coinw multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("coinw precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("coinw precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("coinw min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("coinw min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("coinw fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetTickerRequest;
        let req = GetTickerRequest { instrument: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|x| x.fair_price).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { name: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|x| FundingRate {
            rate: x.settlement_rate,
            time: x.settled_at * 1000,
            interval: x.settled_period * 60 * 60 * 1000,
            premium_interval: 8 * 60 * 60 * 1000,
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
        let start_time = OffsetDateTime::now_utc() - Duration::days(day as i64);
        use crate::futures_api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            instrument: symbol_id,
            day: day + 1,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.created_date.assume_offset(UtcOffset::from_hms(8, 0, 0).unwrap()) > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = (resp[0].created_date - resp[1].created_date).whole_milliseconds() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: (x.created_date.assume_offset(UtcOffset::from_hms(8, 0, 0).unwrap()).unix_timestamp_nanos() / 1_000_000) as u64,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
