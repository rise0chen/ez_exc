use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{self, Order, OrderId};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct PlaceOrderRequest {
    pub symbol: Symbol,
    pub data: order::PlaceOrderRequest,
    pub ch: Sender<Result<OrderId, (OrderId, ExchangeError)>>,
}

#[derive(Debug)]
pub struct CancelOrderRequest {
    pub order: OrderId,
    pub ch: Sender<Result<OrderId, ExchangeError>>,
}

#[derive(Debug)]
pub struct GetOrderRequest {
    pub order: OrderId,
    pub ch: Sender<Result<Order, ExchangeError>>,
}
