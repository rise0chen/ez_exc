use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
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

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Unknown = 0,
    Limit = 1,
    Market = 5,
    PostOnly = 2,
    Ioc = 3,
    Fok = 4,
}
impl From<OrderType> for exc_util::types::order::OrderType {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Unknown => Self::Unknown,
            OrderType::Limit => Self::Limit,
            OrderType::Market => Self::Market,
            OrderType::PostOnly => Self::LimitMaker,
            OrderType::Ioc => Self::ImmediateOrCancel,
            OrderType::Fok => Self::FillOrKill,
        }
    }
}
impl From<exc_util::types::order::OrderType> for OrderType {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Unknown,
            exc_util::types::order::OrderType::Limit => Self::Limit,
            exc_util::types::order::OrderType::Market => Self::Market,
            exc_util::types::order::OrderType::LimitMaker => Self::PostOnly,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Fok,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum OrderStatus {
    #[num_enum(default)]
    Unknown = 0,
    Live = 1,
    Filled = 3,
    PartiallyFilled = 2,
    Canceled = 4,
    MmpCanceled = 5,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::Live => Self::New,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            OrderStatus::Canceled => Self::Canceled,
            OrderStatus::MmpCanceled => Self::PartiallyCanceled,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum FuturesOpenType {
    #[num_enum(default)]
    Cash = 0,
    Isolated = 1,
    Cross = 2,
}
impl From<FuturesOpenType> for exc_util::types::order::FuturesOpenType {
    fn from(value: FuturesOpenType) -> Self {
        match value {
            FuturesOpenType::Cash => Self::Unknown,
            FuturesOpenType::Isolated => Self::Isolated,
            FuturesOpenType::Cross => Self::Cross,
        }
    }
}
impl From<exc_util::types::order::FuturesOpenType> for FuturesOpenType {
    fn from(value: exc_util::types::order::FuturesOpenType) -> Self {
        match value {
            exc_util::types::order::FuturesOpenType::Unknown => Self::Cash,
            exc_util::types::order::FuturesOpenType::Isolated => Self::Isolated,
            exc_util::types::order::FuturesOpenType::Cross => Self::Cross,
        }
    }
}
