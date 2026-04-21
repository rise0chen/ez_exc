use super::Lighter;
use crate::futures_api::types::PositionSide;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Lighter {
    #[allow(unused)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = 1.0;
        let mut multi_size = 1.0;
        let mut precision_size = 0;
        let mut precision_price = 2;
        let mut min_size = 0.0;
        let mut min_usd = 0.0;
        let mut fee = 0.0;

        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { market_id: symbol_id };
        let Some(a) = self.oneshot(req).await?.order_books.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        precision_size = a.supported_size_decimals;
        assert!(precision_size >= 0);
        precision_price = a.supported_price_decimals;
        min_size = a.min_base_amount;
        min_usd = a.min_quote_amount;
        fee = a.taker_fee;

        if symbol.multi_price != multi_price {
            tracing::error!("lighter multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("lighter multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("lighter precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("lighter precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("lighter min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("lighter min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee && fee != 0.0 {
            tracing::warn!("lighter fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

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
            premium_interval: 8 * 60 * 60 * 1000,
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
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = (resp[0].timestamp - resp[1].timestamp) * 1000;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: if matches!(x.direction, PositionSide::Long) { 1.0 } else { -1.0 } * x.rate / 100.0,
                time: x.timestamp * 1000,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
