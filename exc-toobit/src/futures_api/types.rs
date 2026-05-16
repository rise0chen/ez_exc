use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    #[serde(alias = "")]
    Unknown = 0,
    Long = 1,
    Short = 2,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
    #[serde(alias = "")]
    Unknown = 0,
    Buy = 1,
    Sell = 3,
}
impl From<OrderSide> for exc_util::types::order::OrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Unknown => Self::Unknown,
            OrderSide::Buy => Self::Buy,
            OrderSide::Sell => Self::Sell,
        }
    }
}
impl From<exc_util::types::order::OrderSide> for OrderSide {
    fn from(value: exc_util::types::order::OrderSide) -> Self {
        match value {
            exc_util::types::order::OrderSide::Unknown => Self::Unknown,
            exc_util::types::order::OrderSide::Buy => Self::Buy,
            exc_util::types::order::OrderSide::CloseSell => Self::Buy,
            exc_util::types::order::OrderSide::CloseBuy => Self::Sell,
            exc_util::types::order::OrderSide::Sell => Self::Sell,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Unknown = 0,
    Limit = 1,
    Market = 5,
}
impl From<exc_util::types::order::OrderType> for OrderType {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Unknown,
            exc_util::types::order::OrderType::Limit => Self::Limit,
            exc_util::types::order::OrderType::Market => Self::Market,
            exc_util::types::order::OrderType::LimitMaker => Self::Limit,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Limit,
            exc_util::types::order::OrderType::FillOrKill => Self::Limit,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
    PostOnly,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Gtc,
            exc_util::types::order::OrderType::Limit => Self::Gtc,
            exc_util::types::order::OrderType::Market => Self::Ioc,
            exc_util::types::order::OrderType::LimitMaker => Self::PostOnly,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Fok,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Unknown = 0,
    New = 1,
    PartiallyFilled = 2,
    Filled = 3,
    Canceled = 4,
    Rejected = 5,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::New => Self::New,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::Canceled | OrderStatus::Rejected => Self::Canceled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
        }
    }
}
