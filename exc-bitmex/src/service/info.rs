use super::Bitmex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Bitmex {
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
            let req = GetInfoRequest { symbol: symbol_id };
            let Some(a) = self.oneshot(req).await?.pop() else {
                return Err(ExchangeError::OrderNotFound);
            };
            multi_size = 1.0 / a.underlying_to_position_multiplier as f64;
            precision_size = -(a.lot_size as f64).log10().round() as i8;
            precision_price = -a.tick_size.log10().round() as i8;
            min_size = 0.0;
            min_usd = 0.0;
            fee = a.taker_fee;
        }
        if symbol.multi_price != multi_price {
            tracing::error!("bitmex multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("bitmex multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("bitmex precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("bitmex precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("bitmex min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("bitmex min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("bitmex fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        if let Ok(position) = self.get_position(symbol).await {
            symbol.position = position.size;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|x| symbol.token_price(x.indicative_settle_price))
            .ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|x| FundingRate {
            rate: x.indicative_funding_rate,
            time: (x.funding_timestamp.unix_timestamp_nanos() / 1_000_000) as u64,
            interval: (x.funding_interval.unix_timestamp_nanos() / 1_000_000 - 946684800000) as u64,
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
            symbol: symbol_id,
            start_time,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.timestamp > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = (resp[0].timestamp - resp[1].timestamp).whole_milliseconds() as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: (x.timestamp.unix_timestamp_nanos() / 1_000_000) as u64,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
