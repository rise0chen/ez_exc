use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::info::FundingRate;
use hypersdk::hypercore::Dex;
use time::{Duration, OffsetDateTime};

impl Hyperliquid {
    #[allow(unused)]
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let mut multi_price = 1.0;
        let mut multi_size = 1.0;
        let mut precision_size = 0;
        let mut precision_price = 2;

        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            let Some(a) = self.http.spot().await?.into_iter().find(|x| x.name == symbol_id) else {
                return Err(ExchangeError::OrderNotFound);
            };
            precision_size = a.base().sz_decimals as i8;
            precision_price = 8 - precision_size;
        } else {
            let a = if let Some(dex) = crate::symnol::dex(symbol) {
                self.http.perps_from(Dex::new(dex, 0)).await?.into_iter().find(|x| x.name == symbol_id)
            } else {
                self.http.perps().await?.into_iter().find(|x| x.name == symbol_id)
            };
            let Some(a) = a else {
                return Err(ExchangeError::OrderNotFound);
            };
            precision_size = a.sz_decimals as i8;
            precision_price = 6 - precision_size;
        }
        if symbol.multi_price != multi_price {
            tracing::error!("hyperliquid multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size.max(multi_size) / symbol.multi_size.min(multi_size) > 8.0 {
            tracing::error!("hyperliquid multi_size from {} to {}", symbol.multi_size, multi_size);
            symbol.multi_size = multi_size;
        }
        if symbol.precision != precision_size {
            tracing::warn!("hyperliquid precision_size from {} to {}", symbol.precision, precision_size);
            symbol.precision = precision_size;
        }
        if symbol.precision_price != precision_price {
            tracing::warn!("hyperliquid precision_price from {} to {}", symbol.precision_price, precision_price);
            symbol.precision_price = precision_price;
        }
        Ok(())
    }

    pub async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        if symbol.is_spot() {
            return Ok(0.0);
        }
        let coin = crate::symnol::symbol_id(symbol);
        if let Some(ch) = self.ws.index_prices.get(&coin) {
            Ok(*ch.borrow())
        } else {
            Err(ExchangeError::OrderNotFound)
        }
    }

    pub async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError> {
        if symbol.is_spot() {
            return Ok(FundingRate::default());
        }
        let coin = crate::symnol::symbol_id(symbol);
        let rate = if let Some(ch) = self.ws.funding_rates.get(&coin) {
            *ch.borrow()
        } else {
            return Err(ExchangeError::OrderNotFound);
        };
        let interval: u64 = 60 * 60 * 1000;
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        Ok(FundingRate {
            rate,
            time: ((now / interval) + 1) * interval,
            interval,
        })
    }
    pub async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError> {
        if symbol.is_spot() {
            return Ok(Vec::new());
        }
        let start_time = ((OffsetDateTime::now_utc() - Duration::days(day as i64)).unix_timestamp_nanos() / 1_000_000) as u64;
        let coin = crate::symnol::symbol_id(symbol);
        let mut resp = self.http.funding_history(coin, start_time, None).await?;
        resp.retain(|x| x.time > start_time);
        if resp.len() < 2 {
            return Err(ExchangeError::OrderNotFound);
        }
        resp.reverse();
        let interval = resp[0].time - resp[1].time;
        Ok(resp
            .into_iter()
            .map(|x| FundingRate {
                rate: x.funding_rate.as_f64(),
                time: x.time,
                interval,
            })
            .collect())
    }
}
