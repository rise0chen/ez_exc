use super::Toobit;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Toobit {
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
        if symbol.is_spot() {
            todo!()
        } else {
            use crate::futures_api::http::info::{Filter, GetInfoRequest};
            let req = GetInfoRequest {};
            let Some(info) = self.oneshot(req).await?.contracts.into_iter().find(|x| x.symbol == symbol_id) else {
                return Err(ExchangeError::OrderNotFound);
            };
            for f in info.filters {
                match f {
                    Filter::PriceFilter { tick_size } => {
                        precision_price = -tick_size.log10().round() as i8;
                    }
                    Filter::LotSize { step_size, min_qty } => {
                        precision_size = -step_size.log10().round() as i8;
                        min_size = min_qty;
                    }
                    Filter::MinNotional { min_notional } => min_usd = min_notional,
                }
            }
            if info.status != "TRADING" {
                symbol.can_open = false;
            }
        }
        if symbol.multi_price != multi_price {
            tracing::error!("toobit multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("toobit multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("toobit precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("toobit precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("toobit min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("toobit min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee && fee != 0.0 {
            tracing::warn!("toobit fee from {} to {}", symbol.fee, fee);
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
        let symbol_id = crate::symnol::symbol_id(&symbol);
        use crate::futures_api::http::info::GetIndexPriceRequest;
        let req = GetIndexPriceRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.index.pop();
        resp.map(|(_, p)| p).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| FundingRate {
            rate: resp.rate,
            time: resp.next_funding_time,
            interval: 8 * 60 * 60 * 1000,
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
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        use crate::futures_api::http::info::GetFundingRateHistoryRequest;
        let req = GetFundingRateHistoryRequest {
            symbol: symbol_id,
            limit: 24 * day,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.settle_time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].settle_time - resp[1].settle_time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.settle_rate,
                time: x.settle_time,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
