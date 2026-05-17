use super::Dydx;
use bigdecimal::ToPrimitive;
use chrono::{Duration, Utc};
use dydx::indexer::GetHistoricalFundingOpts;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;

impl Dydx {
    #[allow(unused_assignments)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = symbol.parse_prefix();
        let mut multi_size = symbol.multi_size;
        let mut precision_size = symbol.precision;
        let mut precision_price = symbol.precision_price;
        let mut min_size = symbol.min_size;
        let mut min_usd = symbol.min_usd;
        let mut fee = symbol.fee;

        multi_price = 1.0;
        multi_size = 1.0;
        let symbol_id = crate::symnol::symbol_id(symbol);
        let a = self.indexer().markets().get_perpetual_market(&symbol_id).await?;
        precision_size = -a.step_size.to_f64().unwrap().log10().round() as i8;
        precision_price = -a.tick_size.to_f64().unwrap().log10().round() as i8;
        min_size = 0.0;
        min_usd = 0.0;
        let account = self.wallet().account_offline(0)?;
        if let Ok(a) = self.client().await.get_user_fee_tier(account.address().clone()).await {
            fee = a.taker_fee_ppm as f64 / 1e6;
        }

        if symbol.multi_price != multi_price {
            tracing::error!("dydx multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("dydx multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("dydx precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("dydx precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        if symbol.min_size != min_size {
            tracing::warn!("dydx min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("dydx min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("dydx fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer().markets().get_perpetual_market(&symbol_id).await?;
        Ok(resp.oracle_price.map(|x| symbol.token_price(x.to_f64().unwrap())).unwrap_or(0.0))
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.indexer().markets().get_perpetual_market(&symbol_id).await?;
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate: resp.next_funding_rate.to_f64().unwrap(),
            time: ((now / interval) + 1) * interval,
            interval,
            premium_interval: 8 * 60 * 60 * 1000,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let start_time = Utc::now() - Duration::days(day as i64);
        let mut resp = self
            .indexer()
            .markets()
            .get_perpetual_market_historical_funding(
                &symbol_id,
                Some(GetHistoricalFundingOpts {
                    limit: Some(day as u32 * 24),
                    effective_before_or_at: None,
                    effective_before_or_at_height: None,
                }),
            )
            .await?;
        resp.retain(|x| x.effective_at > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        let interval = (resp[0].effective_at.timestamp_millis() - resp[1].effective_at.timestamp_millis()) as u64;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.rate.to_f64().unwrap(),
                time: x.effective_at.timestamp_millis() as u64,
                interval,
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
