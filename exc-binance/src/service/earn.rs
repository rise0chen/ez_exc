use super::Binance;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::{common_st_rate, StRate};
use tower::ServiceExt;

impl Binance {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        if let Some(rate) = common_st_rate(&symbol.base) {
            return Ok(rate);
        }
        let _coin: String = match symbol.base.as_str() {
            "" => {
                return Ok(StRate {
                    rate: 1.0,
                    start_time: 0,
                    apy: 0.0,
                })
            }
            "BNSOL" => {
                use crate::spot_api::http::earn::GetBnsolRequest;
                let req = GetBnsolRequest {};
                let resp = self.oneshot(req).await?.rows;
                let resp = resp.into_iter().max_by_key(|x| x.time).unwrap();
                return Ok(StRate {
                    rate: resp.exchange_rate,
                    start_time: resp.time,
                    apy: resp.annual_percentage_rate,
                });
            }
            "WBETH" => {
                use crate::spot_api::http::earn::GetWbethRequest;
                let req = GetWbethRequest {};
                let resp = self.oneshot(req).await?.rows;
                let resp = resp.into_iter().max_by_key(|x| x.time).unwrap();
                return Ok(StRate {
                    rate: resp.exchange_rate,
                    start_time: resp.time,
                    apy: resp.annual_percentage_rate,
                });
            }
            _ => return Err(ExchangeError::OrderNotFound),
        };
    }
}
