use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Unknown = 0,
    Limit = 1,
    Market = 5,
}
impl From<OrderType> for exc_util::types::order::OrderType {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Unknown => Self::Unknown,
            OrderType::Limit => Self::Limit,
            OrderType::Market => Self::Market,
        }
    }
}
impl From<exc_util::types::order::OrderType> for OrderType {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Unknown,
            exc_util::types::order::OrderType::Limit => Self::Limit,
            exc_util::types::order::OrderType::Market => Self::Market,
            exc_util::types::order::OrderType::LimitMaker => Self::Limit,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Market,
            exc_util::types::order::OrderType::FillOrKill => Self::Market,
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
