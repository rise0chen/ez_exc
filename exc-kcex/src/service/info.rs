use super::Kcex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Kcex {
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
        use crate::futures_web::http::info::GetInfoRequest;
        let req = GetInfoRequest { symbol: symbol_id };
        let Some(a) = self.oneshot(req).await?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        multi_size = a.cs;
        precision_size = 0;
        precision_price = -a.pu.log10().round() as i8;
        min_size = a.min_v;
        min_usd = 0.0;
        fee = a.tfr;

        if symbol.multi_price != multi_price {
            tracing::error!("kcex multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("kcex multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("kcex precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("kcex precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("kcex min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("kcex min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("kcex fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_web::http::info::GetIndexPriceRequest;
        let req = GetIndexPriceRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(symbol.token_price(resp.index_price))
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_web::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(FundingRate {
            rate: resp.funding_rate,
            time: resp.next_settle_time,
            interval: resp.collect_cycle * 60 * 60 * 1000,
            premium_interval: resp.collect_cycle * 60 * 60 * 1000,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        if symbol.is_spot() {
            return Ok(Vec::new());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_web::http::info::GetFundingRateHistoryRequest;
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        let req = GetFundingRateHistoryRequest {
            symbol: symbol_id,
            page_size: day * 24,
        };
        let mut resp = self.oneshot(req).await?.result_list;
        resp.retain(|x| x.settle_time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].settle_time - resp[1].settle_time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.settle_time,
                interval,
                premium_interval: interval,
            })
            .collect())
    }
}
