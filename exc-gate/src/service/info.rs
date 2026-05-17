use super::Gate;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Gate {
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
            use crate::spot_api::http::info::GetInfoRequest;
            let req = GetInfoRequest {
                currency_pair: symbol_id.clone(),
            };
            let a = self.oneshot(req).await?;
            precision_size = a.amount_precision;
            precision_price = a.precision;
            min_size = a.min_base_amount.unwrap_or_default();
            min_usd = a.min_quote_amount.unwrap_or_default();
            use crate::spot_api::http::account::GetFeeRequest;
            let req = GetFeeRequest {
                currency_pair: Some(symbol_id),
            };
            if let Ok(a) = self.oneshot(req).await {
                fee = a.gt_taker_fee;
            }
        } else {
            use crate::futures_api::http::info::GetInfoRequest;
            let req = GetInfoRequest { contract: symbol_id.clone() };
            let a = self.oneshot(req).await?;
            multi_size = a.quanto_multiplier;
            precision_size = 0;
            precision_price = -a.order_price_round.log10().round() as i8;
            min_size = a.order_size_min;
            min_usd = 0.0;
            use crate::spot_api::http::account::GetFeeRequest;
            let req = GetFeeRequest {
                currency_pair: Some(symbol_id),
            };
            if let Ok(a) = self.oneshot(req).await {
                fee = a.futures_taker_fee;
            }
        }
        if symbol.multi_price != multi_price {
            tracing::error!("gate multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("gate multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("gate precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("gate precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("gate min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("gate min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("gate fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { contract: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(symbol.token_price(resp.index_price))
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { contract: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(FundingRate {
            rate: resp.funding_rate,
            time: resp.funding_next_apply * 1000,
            interval: resp.funding_interval * 1000,
            premium_interval: resp.funding_interval * 1000,
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
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000_000) as u64;
        use crate::futures_api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            contract: symbol_id,
            from: Some(start_time),
            to: None,
            limit: Some(day * 24),
        };
        let resp = self.oneshot(req).await?;
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = (resp[0].t - resp[1].t) * 1000;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.r,
                time: x.t * 1000,
                interval,
                premium_interval: interval,
            })
            .collect())
    }
}
