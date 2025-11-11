use crate::api::types::OrderSide;

use super::Bybit;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use tower::ServiceExt;

impl Bybit {
    pub async fn get_balance(&mut self) -> Result<f64, ExchangeError> {
        use crate::api::http::account::GetBalanceRequest;
        let req = GetBalanceRequest {
            account_type: "UNIFIED",
            coin: Some("USDT".to_string()),
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| resp.total_margin_balance).ok_or(ExchangeError::OrderNotFound)
    }
    pub async fn get_position(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError> {
        let symbol_id = crate::symnol::symbol_id(symbol);
        let position = if symbol.is_spot() {
            use crate::api::http::account::GetBalanceRequest;
            let req = GetBalanceRequest {
                account_type: "UNIFIED",
                coin: None,
            };
            let resp = self.oneshot(req).await?.list.pop();
            resp.map(|resp| {
                resp.coin
                    .iter()
                    .find(|x| x.coin == symbol.base.as_str())
                    .map(|x| x.equity)
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
        } else {
            use crate::api::http::account::GetPositionRequest;
            let req = GetPositionRequest {
                category: symbol.kind,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?.list.pop();
            resp.map(|resp| if matches!(resp.side, OrderSide::Buy) { resp.size } else { -resp.size })
                .unwrap_or(0.0)
        };
        Ok(position)
    }
}
