use super::Gate;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Gate {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.total_margin_balance)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::spot_api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                currency: Some(symbol.base.as_str().to_string()),
            };
            let resp = self.oneshot(req).await?;
            resp.0
                .iter()
                .find(|x| x.currency == symbol.base.as_str())
                .map(|x| x.available)
                .unwrap_or(0.0)
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { contract: symbol_id };
            let resp = self.oneshot(req).await?;
            resp.size.unwrap_or(0.0)
        };
        Ok(position)
    }
}
