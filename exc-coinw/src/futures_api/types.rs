use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    Num(u64),
    Str(String),
}
impl Id {
    pub fn into_string(self) -> String {
        match self {
            Id::Num(id) => id.to_string(),
            Id::Str(id) => id,
        }
    }
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::Num(id) => write!(f, "{}", id),
            Id::Str(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionSide {
    #[serde(alias = "")]
    Unknown = 0,
    Long = 1,
    Short = 2,
    Both = 3,
}
impl From<PositionSide> for exc_util::types::order::OrderSide {
    fn from(value: PositionSide) -> Self {
        match value {
            PositionSide::Unknown | PositionSide::Both => Self::Unknown,
            PositionSide::Long => Self::Buy,
            PositionSide::Short => Self::Sell,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeInForce {
    /// 市价
    Execute,
    /// 限价
    Plan,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "PostOnly")]
    PostOnly,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Plan,
            exc_util::types::order::OrderType::Limit => Self::Plan,
            exc_util::types::order::OrderType::Market => Self::Ioc,
            exc_util::types::order::OrderType::LimitMaker => Self::PostOnly,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Fok,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Unknown,
    UnFinish,
    Part,
    Finish,
    Cancel,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Unknown => Self::Unknown,
            OrderStatus::UnFinish => Self::New,
            OrderStatus::Part => Self::PartiallyFilled,
            OrderStatus::Finish => Self::Filled,
            OrderStatus::Cancel => Self::Canceled,
        }
    }
}
