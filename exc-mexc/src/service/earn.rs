use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use time::OffsetDateTime;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        use crate::spot_web::http::earn::GetStRateRequest;
        let project_no: String = match symbol.base.as_str() {
            "MXSOL" => "1928598353273954304".into(),
            _ => return Err(ExchangeError::OrderNotFound),
        };
        let req = GetStRateRequest { project_no };
        let resp = self.oneshot(req).await?;
        let hold_time = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64 - resp.last_epoch_update_time;
        let hold_reward = resp.apy / 365.0 * (hold_time as f64 / (24 * 60 * 60 * 1000) as f64);
        let withdraw = resp.fee + resp.apy / 365.0 * resp.redeem_period;
        let rate = resp.convert_rate * (1.0 + hold_reward) * (1.0 - withdraw);
        Ok(rate)
    }
}
