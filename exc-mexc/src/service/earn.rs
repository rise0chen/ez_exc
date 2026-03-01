use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::earn::StRate;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError> {
        use crate::spot_web::http::earn::GetStRateRequest;
        let project_no: String = match symbol.base.as_str() {
            "PAXG" => {
                return Ok(StRate {
                    rate: 1.01,
                    start_time: 0,
                    apy: 0.0,
                })
            }
            "MXSOL" => "1928598353273954304".into(),
            _ => return Err(ExchangeError::OrderNotFound),
        };
        let req = GetStRateRequest { project_no };
        let resp = self.oneshot(req).await?;
        let apy = resp.apy;
        let fee = resp.fee;
        let withdraw = fee + apy / 365.0 * resp.redeem_period;
        let rate = resp.convert_rate * (1.0 - 0.2 * withdraw);
        Ok(StRate {
            rate,
            start_time: resp.last_epoch_update_time,
            apy,
        })
    }
}
