use super::Htx;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Htx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::spot_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {
            account_id: self.key.account_id,
        };
        let resp = self.oneshot(req).await?.data;
        let balance = resp.list.iter().filter(|x| x.currency == "USDT").map(|x| x.balance).sum();
        Ok(balance)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let position = if symbol.is_spot() {
            use crate::spot_api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                account_id: self.key.account_id,
            };
            let resp = self.oneshot(req).await?.data;
            resp.list.iter().filter(|x| x.currency == symbol.base.as_str()).map(|x| x.balance).sum()
        } else {
            todo!();
        };
        Ok(position)
    }
}
