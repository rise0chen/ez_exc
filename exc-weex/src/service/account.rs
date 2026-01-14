use super::Weex;
use crate::futures_api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Weex {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::futures_api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {};
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| resp.equity).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_positions(&mut self, symbol: &Symbol) -> Result<(f64, f64), ExchangeError> {
        let (mut long, mut short) = (0.0, 0.0);
        let symbol_id = crate::symnol::symbol_id(symbol);
        if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            for x in &resp {
                if x.side == OrderSide::Short {
                    short += x.size
                } else {
                    long += x.size
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
            use crate::futures_api::http::account::GetPositionRequest;
            let req = GetPositionRequest { symbol: symbol_id };
            let resp = self.oneshot(req).await?.0;
            resp.iter().map(|x| if x.side == OrderSide::Short { -x.size } else { x.size }).sum()
        };
        Ok(position)
    }
}
