use super::Aden;
use exc_util::error::ExchangeError;
use exc_util::{symbol::Symbol, types::account::Position};
use tower::ServiceExt;

impl Aden {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.cross_margin_balance)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { contract: symbol_id };
            let resp = self.oneshot(req).await?;
            let size = resp.size.unwrap_or(0.0);
            Position {
                size: symbol.token_size(size),
                price: symbol.token_price(resp.entry_price.unwrap_or(0.0)),
            }
        };
        Ok(position)
    }
}
