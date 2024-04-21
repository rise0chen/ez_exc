use crate::symbol::Symbol;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct PlaceOrderRequest {
    pub size: f64,
    pub price: f64,
    pub kind: OrderType,
    pub leverage: f64,
    pub open_type: FuturesOpenType,
}
impl PlaceOrderRequest {
    pub fn new(size: f64, price: f64, kind: OrderType) -> Self {
        Self {
            size,
            price,
            kind,
            leverage: 1.0,
            open_type: FuturesOpenType::Cross,
        }
    }
    pub fn set_leverage(&mut self, leverage: f64) {
        self.leverage = leverage;
    }
    pub fn set_open_type(&mut self, open_type: FuturesOpenType) {
        self.open_type = open_type;
    }
}

#[derive(Debug)]
pub struct OrderId {
    pub symbol: Symbol,
    pub order_id: Option<String>,
    pub custom_order_id: Option<String>,
}

#[derive(Debug)]
pub struct Order {
    pub symbol: String,
    pub order_id: String,
    pub vol: f64,
    pub deal_vol: f64,
    pub deal_avg_price: f64,
    /// 正数为扣费，负数返费
    pub fee: f64,
    pub state: OrderStatus,
    pub side: OrderSide,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
    Unknown = 0,
    Buy = 1,
    Sell = 3,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderType {
    #[num_enum(default)]
    Unknown = 0,
    Limit = 1,
    Market = 5,
    LimitMaker = 2,
    ImmediateOrCancel = 3,
    FillOrKill = 4,
}
impl Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.serialize(f)
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderStatus {
    #[num_enum(default)]
    Unknown = 0,
    New = 1,
    Filled = 3,
    PartiallyFilled = 2,
    Canceled = 4,
    PartiallyCanceled = 5,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum FuturesOpenType {
    #[num_enum(default)]
    Unknown = 0,
    Isolated = 1,
    Cross = 2,
}
