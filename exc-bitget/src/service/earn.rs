use super::Bitget;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::{common_st_rate, StRate};
use tower::ServiceExt;

impl Bitget {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        if let Some(rate) = common_st_rate(&symbol.base) {
            return Ok(rate);
        }
        let mut ret = StRate {
            rate: 1.0,
            start_time: 0,
            apy: 0.0,
        };
        use crate::api::http::earn::GetEarnRequest;
        let req = GetEarnRequest {};
        let resp = self.oneshot(req).await?;
        match symbol.base.as_str() {
            "BGSOL" => {
                let Some(rate) = resp.iter().find(|x| symbol.base == x.coin) else {
                    return Err(ExchangeError::OrderNotFound);
                };
                ret.apy = (rate.max_apr + rate.min_apr) / 2.0 / 100.0;
                let Some(rate) = rate.subscription_coin_list.iter().find(|x| x.subscription_coin == "SOL") else {
                    return Err(ExchangeError::OrderNotFound);
                };
                ret.rate = rate.exchange_rate;
                ret.start_time = u64::MAX;
            }
            _ => return Err(ExchangeError::OrderNotFound),
        };
        Ok(ret)
    }
}
