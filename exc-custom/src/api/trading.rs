use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{self, AmendOrder, Order, OrderId};
use tokio::sync::oneshot::Sender;

pub struct PlaceOrderRequest {
    pub symbol: Symbol,
    pub data: order::PlaceOrderRequest,
    pub ch: Sender<Result<OrderId, (OrderId, ExchangeError)>>,
}

pub struct AmendOrderRequest {
    pub data: AmendOrder,
    pub ch: Sender<Result<OrderId, ExchangeError>>,
}

pub struct CancelOrderRequest {
    pub order: OrderId,
    pub ch: Sender<Result<OrderId, ExchangeError>>,
}

pub struct GetOrderRequest {
    pub order: OrderId,
    pub ch: Sender<Result<Order, ExchangeError>>,
}
