use super::Custom;
use crate::api::trading::*;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{self, AmendOrder, Order, OrderId};
use tokio::sync::oneshot;
use tower::ServiceExt;

impl Custom {
    pub async fn place_order(&mut self, symbol: &Symbol, data: order::PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let (tx, rx) = oneshot::channel();
        let req = PlaceOrderRequest {
            symbol: symbol.clone(),
            data,
            ch: tx,
        };
        self.oneshot(req.into())
            .await
            .map_err(|e| (OrderId::new(symbol.clone()), ExchangeError::Other(e.into())))?;
        rx.await.map_err(|e| (OrderId::new(symbol.clone()), ExchangeError::Other(e.into())))?
    }
    pub async fn amend_order(&mut self, order: AmendOrder) -> Result<OrderId, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = AmendOrderRequest { data: order, ch: tx };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = CancelOrderRequest { order: order_id, ch: tx };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        if order_id.order_id.is_none() && order_id.custom_order_id.is_none() {
            return Err(ExchangeError::OrderNotFound);
        }
        let (tx, rx) = oneshot::channel();
        let req = GetOrderRequest { order: order_id, ch: tx };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
}
