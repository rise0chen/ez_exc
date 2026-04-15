use super::Aden;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Aden {
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        if symbol.is_spot() {
            return Ok(());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetInfoRequest;
        let req = GetInfoRequest { contract: symbol_id };
        let a = self.oneshot(req).await?;
        if symbol.multi_price != 1.0 {
            tracing::error!("aden contract multi_price from {} to {}", symbol.multi_price, 1.0);
            symbol.multi_price = 1.0;
        }
        if symbol.multi_size != a.quanto_multiplier {
            tracing::error!("aden contract multi_size from {} to {}", symbol.multi_size, a.quanto_multiplier);
            symbol.multi_size = a.quanto_multiplier;
        }
        if symbol.precision != 0 {
            tracing::warn!("aden contract precision from {} to {}", symbol.precision, 0);
            symbol.precision = 0;
        }
        let precision_price = -a.order_price_round.log10().round() as i8;
        if symbol.precision_price != precision_price {
            tracing::warn!("aden contract precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
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
        Ok(resp.index_price)
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
        if resp.is_empty() {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = (resp[0].t - resp[1].t) * 1000;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.r,
                time: x.t * 1000,
                interval,
            })
            .collect())
    }
}
