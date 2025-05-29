use super::Okx;
use exc_core::ExchangeError;
use tower::ServiceExt;

impl Okx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { ccy: "USDT" };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.details.trading).ok_or(ExchangeError::OrderNotFound)
    }
}
