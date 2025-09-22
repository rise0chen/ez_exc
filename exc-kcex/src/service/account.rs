use super::Kcex;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Kcex {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_web::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?;
        Ok(resp.equity)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(f64, f64), ExchangeError> {
        let (mut long, mut short) = (0.0, 0.0);
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_web::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            for x in &resp {
                if x.position_type == 2 {
                    short += x.hold_vol
                } else {
                    long += x.hold_vol
                }
            }
        }
        Ok((long, short))
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_web::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            resp.iter().map(|x| if x.position_type == 2 { -x.hold_vol } else { x.hold_vol }).sum()
        };
        Ok(position)
    }
}
