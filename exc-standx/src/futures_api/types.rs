use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
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
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Alo,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Gtc,
            exc_util::types::order::OrderType::Limit => Self::Gtc,
            exc_util::types::order::OrderType::Market => Self::Ioc,
            exc_util::types::order::OrderType::LimitMaker => Self::Alo,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Ioc,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Unknown,
    Open,
    Filled,
    Canceled,
    Rejected,
    Untriggered,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::Open | OrderStatus::Untriggered => Self::New,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::Canceled | OrderStatus::Rejected => Self::Canceled,
        }
    }
}
