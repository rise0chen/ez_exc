use super::Gate;
use exc_core::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Gate {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.total_margin_balance)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::spot_api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                currency: Some(symbol.base.as_str().to_string()),
            };
            let resp = self.oneshot(req).await?.0;
            let size = resp
                .iter()
                .find(|x| x.currency == symbol.base.as_str())
                .map(|x| x.available)
                .unwrap_or(0.0);
            Position::new(size)
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { contract: symbol_id };
            let resp = self.oneshot(req).await?;
            let size = resp.size.unwrap_or(0.0);
            Position {
                size,
                price: resp.entry_price.unwrap_or(0.0),
            }
        };
        Ok(position)
    }
}
