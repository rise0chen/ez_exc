use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionSide {
    #[serde(alias = "")]
    Unknown = 0,
    Long = 1,
    Short = 2,
    Both = 3,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Limit = 0,
    Market = 1,
}
impl From<exc_util::types::order::OrderType> for OrderType {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Limit,
            exc_util::types::order::OrderType::Limit => Self::Limit,
            exc_util::types::order::OrderType::Market => Self::Market,
            exc_util::types::order::OrderType::LimitMaker => Self::Limit,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Limit,
            exc_util::types::order::OrderType::FillOrKill => Self::Limit,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i8)]
pub enum TimeInForce {
    #[num_enum(default)]
    Ioc = 0,
    Gtc = 1,
    Gtx = 2,
}
impl From<exc_util::types::order::OrderType> for TimeInForce {
    fn from(value: exc_util::types::order::OrderType) -> Self {
        match value {
            exc_util::types::order::OrderType::Unknown => Self::Gtc,
            exc_util::types::order::OrderType::Limit => Self::Gtc,
            exc_util::types::order::OrderType::Market => Self::Ioc,
            exc_util::types::order::OrderType::LimitMaker => Self::Gtx,
            exc_util::types::order::OrderType::ImmediateOrCancel => Self::Ioc,
            exc_util::types::order::OrderType::FillOrKill => Self::Ioc,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    InProgress,
    Pending,
    Open,
    Filled,
    Canceled,
    CanceledPostOnly,
    CanceledReduceOnly,
    CanceledPositionNotAllowed,
    CanceledMarginNotAllowed,
    CanceledTooMuchSlippage,
    CanceledNotEnoughLiquidity,
    CanceledSelfTrade,
    CanceledExpired,
    CanceledOco,
    CanceledChild,
    CanceledLiquidation,
}
impl From<OrderStatus> for exc_util::types::order::OrderStatus {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::InProgress | OrderStatus::Pending | OrderStatus::Open => Self::New,
            OrderStatus::Filled => Self::Filled,
            _ => Self::Canceled,
        }
    }
}
