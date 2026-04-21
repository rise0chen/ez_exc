use super::Okx;
use exc_util::asset::Asset;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Okx {
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
        use crate::api::http::info::GetInfoRequest;
        let req = GetInfoRequest {
            inst_type: if symbol.is_spot() { "SPOT" } else { "SWAP" },
            inst_id: symbol_id.clone(),
        };
        let Some(a) = self.oneshot(req).await?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        multi_price = a.ct_mult.unwrap_or(1.0);
        multi_size = a.ct_val.unwrap_or(1.0);
        precision_size = -a.lot_sz.log10().round() as i8;
        precision_price = -a.tick_sz.log10().round() as i8;
        min_size = a.min_sz;
        use crate::api::http::account::GetFeeRequest;
        let req = if symbol.is_spot() {
            GetFeeRequest {
                inst_type: "SPOT",
                inst_id: Some(crate::symnol::symbol_id(symbol)),
                inst_family: None,
            }
        } else {
            GetFeeRequest {
                inst_type: "SWAP",
                inst_id: None,
                inst_family: Some(crate::symnol::symbol_id(&Symbol::spot(symbol.base.clone(), symbol.quote.clone()))),
            }
        };
        let Some(a) = self.oneshot(req).await?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        fee = -a.fee_group[0].taker;

        if symbol.multi_price != multi_price {
            tracing::error!("okx multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("okx multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("okx precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("okx precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("okx min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("okx min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee && fee != 0.0 {
            tracing::warn!("okx fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let mut symbol = symbol.clone();
        symbol.kind = exc_util::symbol::SymbolKind::Spot;
        if symbol.quote != "USDT" {
            symbol.quote = Asset::usd();
        }
        let symbol_id = crate::symnol::symbol_id(&symbol);
        use crate::api::http::info::GetIndexPriceRequest;
        let req = GetIndexPriceRequest { inst_id: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.idx_px).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { inst_id: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| FundingRate {
            rate: resp.funding_rate,
            time: resp.funding_time,
            interval: resp.next_funding_time - resp.funding_time,
            premium_interval: resp.next_funding_time - resp.funding_time,
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
            inst_id: symbol_id,
            before: Some(start_time),
            after: None,
            limit: Some(day * 24),
        };
        let resp = self.oneshot(req).await?;
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].funding_time - resp[1].funding_time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.funding_time,
                interval,
                premium_interval: interval,
            })
            .collect())
    }
}
