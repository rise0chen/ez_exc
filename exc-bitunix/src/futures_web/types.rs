use num_enum::{FromPrimitive, IntoPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
    Unknown = 0,
    Buy = 2,
    Sell = 1,
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
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
#[repr(i8)]
pub enum TimeInForce {
    #[num_enum(default)]
    Gtc = 1,
    Ioc = 3,
    Fok = 2,
    PostOnly = 4,
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
