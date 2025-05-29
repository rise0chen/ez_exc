use super::Bybit;
use exc_core::ExchangeError;
use tower::ServiceExt;

impl Bybit {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {
            account_type: "UNIFIED",
            coin: "USDT",
        };
        let resp = self.oneshot(req).await?;
        Ok(resp.balance.wallet_balance)
    }
}
