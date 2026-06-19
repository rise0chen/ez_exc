use num_enum::{FromPrimitive, IntoPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum PositionSide {
    #[default]
    Open = 0,
    Close = 1,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OrderSide {
    #[default]
    Buy = 0,
    Sell = 1,
}
impl OrderSide {
    pub fn is_buy(&self) -> bool {
        matches!(self, OrderSide::Buy)
    }
}
impl From<OrderSide> for exc_util::types::order::OrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Buy => Self::Buy,
            OrderSide::Sell => Self::Sell,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum TimeInForce {
    #[default]
    GoodTillCancel = 0,
    ImmediateOrCancel = 1,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::GoodTillCancel,
            exc_util::types::order::OrderType::Limit => Self::GoodTillCancel,
            exc_util::types::order::OrderType::Market => Self::ImmediateOrCancel,
            exc_util::types::order::OrderType::LimitMaker => Self::GoodTillCancel,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::ImmediateOrCancel,
            exc_util::types::order::OrderType::FillOrKill => Self::ImmediateOrCancel,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OrderStatus {
    #[default]
    Unknown = 0,
    Filled = 1,
    PartiallyCanceled = 3,
    New = 4,
    Canceled = 6,
    Rejected,
    PartiallyFilled,
    PendingCancel,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::New,
            OrderStatus::New => Self::New,
            OrderStatus::Rejected => Self::Canceled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            OrderStatus::PartiallyCanceled => Self::PartiallyCanceled,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PendingCancel => Self::New,
            OrderStatus::Canceled => Self::Canceled,
        }
    }
}
