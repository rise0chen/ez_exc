use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    PostOnly,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::GTC,
            exc_util::types::order::OrderType::Limit => Self::GTC,
            exc_util::types::order::OrderType::Market => Self::IOC,
            exc_util::types::order::OrderType::LimitMaker => Self::PostOnly,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::IOC,
            exc_util::types::order::OrderType::FillOrKill => Self::FOK,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OrderStatus {
    Unknown = 0,
    New = 1,
    Filled = 3,
    PartiallyFilled = 2,
    Cancelled = 4,
    PartiallyFilledCanceled = 5,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::New => Self::New,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            OrderStatus::Cancelled => Self::Canceled,
            OrderStatus::PartiallyFilledCanceled => Self::PartiallyCanceled,
        }
    }
}
