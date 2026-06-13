use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;

pub fn serialize_filter<S>(filter: &BTreeMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_str = serde_json::to_string(filter).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_str)
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PositionSide {
    OneWay,
    Long,
    Short,
}
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TimeInForce {
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::GoodTillCancel,
            exc_util::types::order::OrderType::Limit => Self::GoodTillCancel,
            exc_util::types::order::OrderType::Market => Self::ImmediateOrCancel,
            exc_util::types::order::OrderType::LimitMaker => Self::GoodTillCancel,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::ImmediateOrCancel,
            exc_util::types::order::OrderType::FillOrKill => Self::FillOrKill,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    PendingNew,
    New,
    Rejected,
    PartiallyFilled,
    Filled,
    PendingCancel,
    Canceled,
    PendingReplace,
    Expired,
    DoneForDay,
    Stopped,
    Suspended,
    Calculated,
    AcceptedForBidding,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::PendingNew => Self::New,
            OrderStatus::New => Self::New,
            OrderStatus::Rejected => Self::Canceled,
            OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            OrderStatus::Filled => Self::Filled,
            OrderStatus::PendingCancel => Self::New,
            OrderStatus::Canceled => Self::Canceled,
            OrderStatus::PendingReplace => Self::New,
            OrderStatus::Expired => Self::Expired,
            OrderStatus::DoneForDay => Self::New,
            OrderStatus::Stopped => Self::Canceled,
            OrderStatus::Suspended => Self::New,
            OrderStatus::Calculated => Self::New,
            OrderStatus::AcceptedForBidding => Self::New,
        }
    }
}
