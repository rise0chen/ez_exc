use super::Paradex;
use core::time::Duration;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;

impl Paradex {
    #[allow(unused)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = 1.0;
        let mut multi_size = 1.0;
        let mut precision_size = 0;
        let mut precision_price = 2;

        let symbol_id = crate::symnol::symbol_id(symbol);
        let a = self.http.markets(Some(symbol_id)).await;
        let Some(a) = a.map_err(|e| ExchangeError::Other(e.into()))?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        precision_size = -a.order_size_increment.log10().round() as i8;
        precision_price = -a.price_tick_size.log10().round() as i8;

        if symbol.multi_price != multi_price {
            tracing::error!("paradex multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size != multi_size {
            tracing::error!("paradex multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("paradex precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("paradex precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol = crate::symnol::symbol_id(symbol);
        let resp = self.http.markets_summary(symbol).await.map_err(|e| ExchangeError::Other(e.into()))?.pop();
        resp.map(|resp| resp.mark_price).ok_or(ExchangeError::OrderNotFound)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol = crate::symnol::symbol_id(symbol);
        let resp = self.http.markets_summary(symbol).await.map_err(|e| ExchangeError::Other(e.into()))?.pop();
        let rate = resp.map(|resp| resp.funding_rate).ok_or(ExchangeError::OrderNotFound)?;
        let interval: u64 = 5 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: rate.unwrap_or(0.0) / (8 * 60 * 60 * 1000 / interval) as f64,
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        use crate::futures::http::info::GetFundingRateHistoryResponse;
        let symbol = crate::symnol::symbol_id(symbol);
        let start = chrono::Utc::now() - Duration::from_hours(24 * day as u64);
        let req = vec![("market".into(), symbol.clone())];
        let resp = self
            .http
            .request_cursor::<GetFundingRateHistoryResponse>("/v1/funding/data".into(), Some(req), Some(start), None, false)
            .await
            .map_err(|e| ExchangeError::Other(e.into()))?;
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = resp[0].created_at - resp[1].created_at;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate_8h / (8 * 60 * 60 * 1000 / interval) as f64,
                time: x.created_at,
                interval,
            })
            .collect())
    }
}
