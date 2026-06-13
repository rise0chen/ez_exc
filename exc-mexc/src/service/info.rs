use super::Mexc;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Mexc {
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
            use crate::spot_web::http::info::GetInfoRequest;
            let req = GetInfoRequest {
                symbol: format!("{}_{}", symbol.base, symbol.quote),
            };
            let a = self.oneshot(req).await?;
            symbol.base_id = a.cd;
            symbol.quote_id = a.mcd;
            multi_price = 1.0;
            multi_size = 1.0;
            precision_size = a.qs;
            precision_price = a.ps;
            min_size = 0.0;
            min_usd = a.mi;
            fee = a.tfr;
        } else {
            use crate::futures_api::http::info::GetInfoRequest;
            let req = GetInfoRequest { symbol: symbol_id.clone() };
            let a = self.oneshot(req).await?;
            multi_size = a.contract_size;
            precision_size = a.vol_scale;
            precision_price = a.price_scale;
            min_size = a.min_vol;
            min_usd = 5.0;
            fee = a.taker_fee_rate;
            symbol.can_open = a.state == 0;
        }
        if symbol.multi_price != multi_price {
            tracing::error!("mexc multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("mexc multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("mexc precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("mexc precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("mexc min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("mexc min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("mexc fee from {} to {}", symbol.fee, fee);
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
