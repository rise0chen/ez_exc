pub mod book;
pub mod info;
pub mod trading;

#[derive(Debug)]
pub enum Request {
    GetDepth(book::GetDepthRequest),
    GetFundingRate(info::GetFundingRateRequest),
    GetFundingRateHistory(info::GetFundingRateHistoryRequest),
    PlaceOrder(trading::PlaceOrderRequest),
    AmendOrder(trading::AmendOrderRequest),
    CancelOrder(trading::CancelOrderRequest),
    GetOrder(trading::GetOrderRequest),
}
impl From<book::GetDepthRequest> for Request {
    fn from(value: book::GetDepthRequest) -> Self {
        Self::GetDepth(value)
    }
}
impl From<info::GetFundingRateRequest> for Request {
    fn from(value: info::GetFundingRateRequest) -> Self {
        Self::GetFundingRate(value)
    }
}
impl From<info::GetFundingRateHistoryRequest> for Request {
    fn from(value: info::GetFundingRateHistoryRequest) -> Self {
        Self::GetFundingRateHistory(value)
    }
}
impl From<trading::PlaceOrderRequest> for Request {
    fn from(value: trading::PlaceOrderRequest) -> Self {
        Self::PlaceOrder(value)
    }
}
impl From<trading::AmendOrderRequest> for Request {
    fn from(value: trading::AmendOrderRequest) -> Self {
        Self::AmendOrder(value)
    }
}
impl From<trading::CancelOrderRequest> for Request {
    fn from(value: trading::CancelOrderRequest) -> Self {
        Self::CancelOrder(value)
    }
}
impl From<trading::GetOrderRequest> for Request {
    fn from(value: trading::GetOrderRequest) -> Self {
        Self::GetOrder(value)
    }
}
