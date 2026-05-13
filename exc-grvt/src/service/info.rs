use super::Grvt;
use core::time::Duration;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use grvt_rust_sdk::types::{FundingRequest, InstrumentRequest};

impl Grvt {
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
        let a = self.http.instrument_full(&InstrumentRequest { instrument: symbol_id }).await;
        let a = a.map_err(|e| ExchangeError::Other(e.into()))?.result;
        symbol.base_id = a.instrument_hash;
        symbol.quote_id = a.base_decimals.to_string();
        precision_size = -a.min_size.log10().round() as i8;
        precision_price = -a.tick_size.log10().round() as i8;
        min_size = a.min_size;
        min_usd = a.min_notional;
        let a = self.http.funding_account_summary_full().await;
        let a = a.map_err(|e| ExchangeError::Other(e.into()))?.tier;
        fee = a.futures_taker_fee / 1e6;

        if symbol.multi_price != multi_price {
            tracing::error!("grvt multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("grvt multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("grvt precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("grvt precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("grvt min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("grvt min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee && fee != 0.0 {
            tracing::warn!("grvt fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let req = InstrumentRequest { instrument: symbol_id };
        let resp = self.http.ticker_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?.result;
        Ok(resp.index_price)
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let req = InstrumentRequest { instrument: symbol_id };
        let resp = self.http.ticker_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?.result;
        let rate = resp.funding_rate.unwrap_or_default() / 100.0;
        let time = (resp.next_funding_time.unwrap_or_default() / 1_000_000) as u64;
        let interval: u64 = 8 * 60 * 60 * 1000;
        Ok(FundingRate {
            rate,
            time,
            interval,
            premium_interval: 8 * 60 * 60 * 1000,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let start = chrono::Utc::now() - Duration::from_hours(24 * day as u64);
        let req = FundingRequest {
            instrument: symbol_id,
            start_time: Some(start.timestamp_nanos_opt().unwrap() as u128),
            end_time: None,
            limit: Some(24 * day as u32),
        };
        let resp = self.http.funding_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?.result;
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = ((resp[0].funding_time - resp[1].funding_time) / 1_000_000) as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate / 100.0,
                time: (x.funding_time / 1_000_000) as u64,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
