use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Mexc {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.equity)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::spot_api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest;
            let resp = self.oneshot(req).await?;
            resp.balances
                .iter()
                .find(|x| x.asset == symbol.base.as_str())
                .map(|x| x.free)
                .unwrap_or(0.0)
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0.pop();
            resp.map(|resp| if resp.position_type == 2 { -resp.hold_vol } else { resp.hold_vol })
                .unwrap_or(0.0)
        };
        Ok(position)
    }
}
