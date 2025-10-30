use super::Bitget;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Bitget {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.eff_equity)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {};
            let resp = self.oneshot(req).await?;
            resp.assets
                .iter()
                .find(|x| x.coin == symbol.base.as_str())
                .map(|x| x.balance)
                .unwrap_or(0.0)
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                category: "USDT-FUTURES",
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?.list.unwrap_or_default();
            resp.iter().map(|x| if x.pos_side == "short" { -x.total } else { x.total }).sum()
        };
        Ok(position)
    }
}
