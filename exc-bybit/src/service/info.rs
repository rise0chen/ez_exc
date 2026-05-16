use super::Bybit;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Bybit {
    #[allow(unused_assignments)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetInfoRequest;
        let req = GetInfoRequest {
            category: symbol.kind,
            symbol: symbol_id.clone(),
        };
        let Some(a) = self.oneshot(req).await?.list.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let multi_size = 1.0;
        let precision_size = a.lot_size_filter.qty_step.or(a.lot_size_filter.base_precision).unwrap_or(1.0);
        let precision_size = -precision_size.log10().round() as i8;
        let precision_price = -a.price_filter.tick_size.log10().round() as i8;
        let min_size = a.lot_size_filter.min_order_qty;
        let min_usd = a.lot_size_filter.min_notional_value.or(a.lot_size_filter.min_order_amt).unwrap_or(0.0);
        use crate::api::http::account::GetFeeRequest;
        let req = GetFeeRequest {
            category: symbol.kind,
            symbol: symbol_id,
        };
        let fee = self.oneshot(req).await?.list.pop().map(|x| x.taker_fee_rate).unwrap_or(0.0);

        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("bybit multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("bybit precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("bybit precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("bybit min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("bybit min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("bybit fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }
    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest {
            category: symbol.kind,
            symbol: symbol_id,
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| resp.index_price).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest {
            category: symbol.kind,
            symbol: symbol_id,
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.next_funding_time,
            interval: resp.funding_interval_hour * 60 * 60 * 1000,
            premium_interval: resp.funding_interval_hour * 60 * 60 * 1000,
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
        use crate::api::http::info::GetFundingRateHistoryRequest;
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        let req = GetFundingRateHistoryRequest {
            category: symbol.kind,
            symbol: symbol_id,
            start_time: None,
            end_time: None,
            limit: Some(day * 24),
        };
        let mut resp = self.oneshot(req).await?.list;
        resp.retain(|x| x.funding_rate_timestamp > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].funding_rate_timestamp - resp[1].funding_rate_timestamp;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.funding_rate_timestamp,
                interval,
                premium_interval: interval,
            })
            .collect())
    }
}
