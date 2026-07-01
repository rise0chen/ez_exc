use super::Paradex;
use crate::futures::http::info::{GetAccountInfoResponse, GetFundingRateHistoryResponse, TierInfo};
use core::time::Duration;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use time::OffsetDateTime;

impl Paradex {
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
        let a = self.http.markets(Some(symbol_id)).await;
        let Some(a) = a.map_err(|e| ExchangeError::Other(e.into()))?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        multi_size = 1.0;
        precision_size = -a.order_size_increment.log10().round() as i8;
        precision_price = -a.price_tick_size.log10().round() as i8;
        min_size = 0.0;
        min_usd = a.min_notional;
        fee = a
            .fee_config
            .map(|x| if self.key.pro { x.api_fee } else { x.interactive_fee }.taker_fee.fee)
            .unwrap_or(0.0);
        if let Some(a) = self
            .http
            .request_cursor::<GetAccountInfoResponse>("/v1/account/info".into(), None, None, None, true)
            .await
            .ok()
            .and_then(|mut x| x.pop())
        {
            if let Ok(tiers) = self
                .http
                .request_cursor::<TierInfo>("/v1/system/volume-tiers".into(), None, None, None, false)
                .await
            {
                let tier = tiers.iter().find(|x| {
                    if self.key.pro {
                        x.kind == "pro" && x.tier_name == a.volume_tiers.pro.tier
                    } else {
                        x.kind == "retail" && x.tier_name == a.volume_tiers.retail.tier
                    }
                });
                if let Some(tier) = tier {
                    fee = tier.fee_rates.taker_rate_rate;
                }
            }
        };

        if symbol.multi_price != multi_price {
            tracing::error!("paradex multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
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
        if symbol.min_size != min_size {
            tracing::warn!("paradex min_size from {} to {}", symbol.min_size, min_size);
            symbol.min_size = min_size;
        }
        if symbol.min_usd != min_usd {
            tracing::warn!("paradex min_usd from {} to {}", symbol.min_usd, min_usd);
            symbol.min_usd = min_usd;
        }
        if symbol.fee != fee {
            tracing::warn!("paradex fee from {} to {}", symbol.fee, fee);
            symbol.fee = fee;
        }
        if let Ok(position) = self.get_position(symbol).await {
            symbol.position = position.size;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let resp = self.http.markets_summary(symbol_id).await;
        let Some(resp) = resp.map_err(|e| ExchangeError::Other(e.into()))?.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        Ok(symbol.token_price(resp.underlying_price.unwrap_or(resp.mark_price)))
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
            premium_interval: 8 * 60 * 60 * 1000,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
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
                premium_interval: 8 * 60 * 60 * 1000,
            })
            .collect())
    }
}
