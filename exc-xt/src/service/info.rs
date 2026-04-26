use super::Xt;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Xt {
    #[allow(unused)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = 1.0;
        let mut multi_size = 1.0;
        let mut precision_size = 0;
        let mut precision_price = 2;
        let mut min_size = 0.0;
        let mut min_usd = 0.0;
        let mut fee = 0.0;
        let mut can_open = true;

        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::info::GetInfoRequest;
            let req = GetInfoRequest { symbol: symbol_id };
            let a = self.oneshot(req).await?;
            multi_size = a.contract_size;
            precision_size = a.quantity_precision;
            precision_price = a.price_precision;
            min_size = a.min_qty;
            min_usd = a.min_notional;
            fee = a.taker_fee;
            can_open = a.open_switch;
        }

        if symbol.multi_price != multi_price {
            tracing::error!("xt multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("xt multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("xt precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("xt precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("xt min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("xt min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee && fee != 0.0 {
            tracing::warn!("xt fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        if symbol.can_open != can_open {
            tracing::warn!("xt can_open from {} to {}", symbol.can_open, can_open);
            symbol.can_open = can_open;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetIndexPriceRequest;
        let req = GetIndexPriceRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(resp.p)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?;
        Ok(FundingRate {
            rate: resp.funding_rate,
            time: resp.next_collection_time,
            interval: resp.collection_internal * 60 * 60 * 1000,
            premium_interval: resp.collection_internal * 60 * 60 * 1000,
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
            limit: 24 * day,
        };
        let mut resp = self.oneshot(req).await?.items;
        resp.retain(|x| x.created_time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].created_time - resp[1].created_time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.created_time,
                interval,
                premium_interval: interval,
            })
            .collect())
    }
}
