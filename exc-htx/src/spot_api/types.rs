use exc_util::types::order::OrderSide;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    BuyMarket,
    SellMarket,
    BuyLimit,
    SellLimit,
    BuyIoc,
    SellIoc,
    BuyLimitMaker,
    SellLimitMaker,
    BuyStopLimit,
    SellStopLimit,
    BuyLimitFok,
    SellLimitFok,
    BuyStopLimitFok,
    SellStopLimitFok,
}
impl From<(OrderSide, exc_util::types::order::OrderType)> for OrderType {
    fn from(value: (OrderSide, exc_util::types::order::OrderType)) -> Self {
        match value {
            (OrderSide::Unknown, _) => todo!(),
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::Unknown) => Self::BuyLimit,
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::Limit) => Self::BuyLimit,
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::Market) => Self::BuyMarket,
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::LimitMaker) => Self::BuyLimitMaker,
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::ImmediateOrCancel) => Self::BuyIoc,
            (OrderSide::Buy | OrderSide::CloseSell, exc_util::types::order::OrderType::FillOrKill) => Self::BuyLimitFok,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::Unknown) => Self::SellLimit,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::Limit) => Self::SellLimit,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::Market) => Self::SellMarket,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::LimitMaker) => Self::SellLimitMaker,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::ImmediateOrCancel) => Self::SellIoc,
            (OrderSide::Sell | OrderSide::CloseBuy, exc_util::types::order::OrderType::FillOrKill) => Self::SellLimitFok,
        }
    }
}
impl From<OrderType> for OrderSide {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::BuyMarket => Self::Buy,
            OrderType::SellMarket => Self::Sell,
            OrderType::BuyLimit => Self::Buy,
            OrderType::SellLimit => Self::Sell,
            OrderType::BuyIoc => Self::Buy,
            OrderType::SellIoc => Self::Sell,
            OrderType::BuyLimitMaker => Self::Buy,
            OrderType::SellLimitMaker => Self::Sell,
            OrderType::BuyStopLimit => Self::Buy,
            OrderType::SellStopLimit => Self::Sell,
            OrderType::BuyLimitFok => Self::Buy,
            OrderType::SellLimitFok => Self::Sell,
            OrderType::BuyStopLimitFok => Self::Buy,
            OrderType::SellStopLimitFok => Self::Sell,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    Created,
    Submitted,
    PartialFilled,
    Filled,
    PartialCanceled,
    Canceling,
    Canceled,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Created => Self::New,
            OrderStatus::Submitted => Self::New,
            OrderStatus::PartialFilled => Self::PartiallyFilled,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PartialCanceled => Self::PartiallyCanceled,
            OrderStatus::Canceling => Self::PartiallyFilled,
            OrderStatus::Canceled => Self::Canceled,
        }
    }
}
