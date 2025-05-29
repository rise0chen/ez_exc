use super::Mexc;
use exc_core::ExchangeError;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.equity)
    }
}
