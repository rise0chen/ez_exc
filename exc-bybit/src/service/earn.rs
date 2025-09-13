use super::Bybit;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Bybit {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        use crate::api::http::earn::GetStRateRequest;
        let coin: String = match symbol.base.as_str() {
            "BBSOL" => "BBSOL".into(),
            _ => return Err(ExchangeError::OrderNotFound),
        };
        let req = GetStRateRequest {
            category: "OnChain".into(),
            coin,
        };
        let Some(resp) = self.oneshot(req).await?.list.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let apy: f64 = resp.estimate_apr[..resp.estimate_apr.len() - 1].parse().unwrap_or(0.0) / 100.0;
        let fee = (1.0 / resp.stake_exchange_rate) / resp.redeem_exchange_rate - 1.0;
        let withdraw = fee + apy / 365.0 * (resp.redeem_processing_minute / 60.0 / 24.0);
        let rate = (1.0 / resp.stake_exchange_rate) * (1.0 - 0.1 * withdraw);
        Ok(rate)
    }
}
