use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Bid,
    Ask,
}
impl OrderSide {
    pub fn is_sell(&self) -> bool {
        matches!(self, OrderSide::Ask)
    }
}
impl From<OrderSide> for exc_util::types::order::OrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Bid => Self::Buy,
            OrderSide::Ask => Self::Sell,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Alo,
    Tob,
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

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Unknown,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::Open => Self::New,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            OrderStatus::Cancelled | OrderStatus::Rejected => Self::Canceled,
        }
    }
}
