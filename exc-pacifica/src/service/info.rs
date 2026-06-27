use super::Pacifica;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

impl Pacifica {
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
            fee = 0.0004;
        } else {
            use crate::futures_api::http::info::GetInfoRequest;
            let req = GetInfoRequest { symbol: symbol_id.clone() };
            let Some(info) = self.oneshot(req).await?.into_iter().find(|x| x.symbol == symbol_id) else {
                return Err(ExchangeError::OrderNotFound);
            };
            multi_size = 1.0;
            precision_price = -info.tick_size.log10().round() as i8;
            precision_size = -info.lot_size.log10().round() as i8;
            min_usd = info.min_order_size;

            use crate::futures_api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                account: self.key.account.to_string(),
            };
            if let Ok(a) = self.oneshot(req).await {
                fee = a.taker_fee;
            }
        }
        if symbol.multi_price != multi_price {
            tracing::error!("pacifica multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("pacifica multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("pacifica precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("pacifica precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("pacifica min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("pacifica min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("pacifica fee from {} to {}", symbol.fee, fee);
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
        let req = GetFundingRateRequest { symbol: symbol_id.clone() };
        let resp = self.oneshot(req).await?;
        let Some(resp) = resp.iter().find(|x| x.symbol == symbol_id) else {
            return Err(ExchangeError::OrderNotFound);
        };
        Ok(symbol.token_price(resp.oracle))
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let symbol_id = crate::symnol::symbol_id(symbol);
        use crate::futures_api::http::info::GetFundingRateRequest;
        let req = GetFundingRateRequest { symbol: symbol_id.clone() };
        let resp = self.oneshot(req).await?;
        let Some(resp) = resp.iter().find(|x| x.symbol == symbol_id) else {
            return Err(ExchangeError::OrderNotFound);
        };
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.next_funding,
            time: ((now / interval) + 1) * interval,
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
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000_000) as u64;
        let req = GetFundingRateHistoryRequest {
            symbol: symbol_id,
            limit: 24 * day,
        };
        let mut resp = self.oneshot(req).await?;
        resp.retain(|x| x.created_at > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].created_at - resp[1].created_at;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate,
                time: x.created_at,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
