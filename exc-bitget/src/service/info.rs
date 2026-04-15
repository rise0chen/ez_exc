use super::Bitget;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Bitget {
    #[allow(unused)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = 1.0;
        let mut multi_size = 1.0;
        let mut precision_size = 0;
        let mut precision_price = 2;

        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetInfoRequest;
        let req = GetInfoRequest {
            category: if symbol.is_spot() { "SPOT" } else { "USDT-FUTURES" },
            symbol: symbol_id,
        };
        let Some(a) = self.oneshot(req).await?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        precision_size = a.quantity_precision;
        precision_price = a.price_precision;

        if symbol.multi_price != multi_price {
            tracing::error!("bitget multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("bitget multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("bitget precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("bitget precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetIndexPriceRequest;
        let req = GetIndexPriceRequest {
            symbol: symbol_id,
            product_type: "USDT-FUTURES",
        };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.index_price).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.next_update,
            interval: resp.funding_rate_interval * 60 * 60 * 1000,
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

        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        use crate::api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            category: "USDT-FUTURES",
            symbol: symbol_id,
            limit: day * 24,
        };
        let mut resp = self.oneshot(req).await?.result_list;
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
            })
            .collect())
    }
}
