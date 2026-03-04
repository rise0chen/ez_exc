use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FuturesOpenType {
    Unknown = 0,
    Isolated = 1,
    Cross = 2,
}
impl From<exc_util::types::order::FuturesOpenType> for FuturesOpenType {
    fn from(value: exc_util::types::order::FuturesOpenType) -> Self {
        match value {
            exc_util::types::order::FuturesOpenType::Unknown => Self::Unknown,
            exc_util::types::order::FuturesOpenType::Isolated => Self::Isolated,
            exc_util::types::order::FuturesOpenType::Cross => Self::Cross,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
}
impl From<exc_util::types::order::OrderType> for OrderType {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Limit,
            exc_util::types::order::OrderType::Limit => Self::Limit,
            exc_util::types::order::OrderType::Market => Self::Market,
            exc_util::types::order::OrderType::LimitMaker => Self::PostOnly,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Limit,
            exc_util::types::order::OrderType::FillOrKill => Self::Limit,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Gtc,
            exc_util::types::order::OrderType::Limit => Self::Gtc,
            exc_util::types::order::OrderType::Market => Self::Ioc,
            exc_util::types::order::OrderType::LimitMaker => Self::Gtc,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Fok,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum OrderStatus {
    #[num_enum(default)]
    #[serde(alias = "")]
    Unknown = 0,
    Init = 1,
    New = 2,
    Submitted = 3,
    PartialFilled = 4,
    Filled = 6,
    PartialCanceled = 5,
    Canceling = 11,
    Canceled = 7,
    Rejected = 99,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::Init => Self::New,
            OrderStatus::New => Self::New,
            OrderStatus::Submitted => Self::New,
            OrderStatus::PartialFilled => Self::PartiallyFilled,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PartialCanceled => Self::PartiallyCanceled,
            OrderStatus::Canceling => Self::PartiallyFilled,
            OrderStatus::Canceled => Self::Canceled,
            OrderStatus::Rejected => Self::Canceled,
        }
    }
}
