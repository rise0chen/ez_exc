use super::Extended;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Extended {
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
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { market: symbol_id.clone() };
        let Some(info) = self.oneshot(req).await?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        multi_size = 1.0;
        precision_price = -info.trading_config.min_price_change.log10().round() as i8;
        precision_size = -info.trading_config.min_order_size_change.log10().round() as i8;
        min_size = info.trading_config.min_order_size;
        symbol.base_id = info.l2_config.synthetic_id;
        symbol.base_precision = info.l2_config.synthetic_resolution.log10().round() as i8;
        symbol.quote_id = info.l2_config.collateral_id;
        symbol.quote_precision = info.l2_config.collateral_resolution.log10().round() as i8;
        match &*info.status {
            "ACTIVE" => {
                symbol.can_trade = true;
                symbol.can_open = true;
            }
            "REDUCE_ONLY" => {
                symbol.can_trade = true;
                symbol.can_open = false;
            }
            _ => {
                symbol.can_trade = false;
                symbol.can_open = false;
            }
        }

        use crate::futures_api::http::account::GetFeeRequest;
        let req = GetFeeRequest {
            market: symbol_id,
            builder_id: None,
        };
        if let Ok(a) = self.oneshot(req).await {
            if let Some(a) = a.first() {
                fee = a.taker_fee_rate + a.builder_fee_rate;
            }
        }

        if symbol.multi_price != multi_price {
            tracing::error!("extended multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("extended multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("extended precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("extended precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("extended min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("extended min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("extended fee from {} to {}", symbol.fee, fee);
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
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { market: symbol_id.clone() };
        let resp = self.oneshot(req).await?;
        Ok(symbol.token_price(resp.index_price))
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { market: symbol_id.clone() };
        let resp = self.oneshot(req).await?;
        let interval: u64 = 60 * 60 * 1000;
        Ok(FundingRate {
            rate: resp.funding_rate,
            time: resp.next_funding_rate,
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
        let end_time = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        let start_time = end_time - Duration::days(day as i64).whole_milliseconds() as u64;
        let req = GetFundingRateHistoryRequest {
            market: symbol_id,
            start_time,
            end_time,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.t > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].t - resp[1].t;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.f,
                time: x.t,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
