use super::Okx;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Okx {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest { ccy: Some("USDT".into()) };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.adj_eq).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                ccy: Some(symbol.base.as_str().into()),
            };
            let resp = self.oneshot(req).await?.pop();
            resp.map(|resp| {
                resp.details
                    .iter()
                    .find(|x| x.ccy == symbol.base.as_str())
                    .map(|x| x.avail_bal)
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest { inst_id: symbol_id };
            let resp = self.oneshot(req).await?.pop();
            resp.map(|resp| if resp.pos_side == "short" { -resp.pos } else { resp.pos })
                .unwrap_or(0.0)
        };
        Ok(position)
    }
}
