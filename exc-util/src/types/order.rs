use crate::symbol::Symbol;
use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct PlaceOrderRequest {
    pub size: Decimal,
    pub price: Decimal,
    pub kind: OrderType,
    pub leverage: f64,
    pub open_type: FuturesOpenType,
}
impl PlaceOrderRequest {
    pub fn new(size: Decimal, price: Decimal, kind: OrderType) -> Self {
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

#[derive(Debug, Clone)]
pub struct OrderId {
    pub symbol: Symbol,
    pub order_id: Option<String>,
    pub custom_order_id: Option<String>,
}
impl OrderId {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            order_id: None,
            custom_order_id: None,
        }
    }
}

#[derive(Debug)]
pub struct AmendOrder {
    pub id: OrderId,
    pub size: Option<f64>,
    pub price: Option<f64>,
}

#[derive(Debug)]
pub enum Fee {
    /// 交易货币
    Base(f64),
    /// 计价货币 如USDT
    Quote(f64),
}

#[derive(Debug)]
pub struct Order {
    pub symbol: String,
    pub order_id: String,
    pub vol: f64,
    pub deal_vol: f64,
    pub deal_avg_price: f64,
    /// 正数为扣费，负数返费
    pub fee: Fee,
    pub state: OrderStatus,
    pub side: OrderSide,
}
impl Order {
    pub fn fee_base(&self) -> f64 {
        match self.fee {
            Fee::Base(s) => s,
            Fee::Quote(_) => 0.0,
        }
    }
    pub fn fee_quote(&self) -> f64 {
        match self.fee {
            Fee::Base(s) => s * self.deal_avg_price,
            Fee::Quote(s) => s,
        }
    }
}

#[derive(FromPrimitive, IntoPrimitive)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i8)]
pub enum OrderSide {
    #[num_enum(default)]
    Unknown = 0,
    Buy = 1,
    CloseSell = 2,
    Sell = 3,
    CloseBuy = 4,
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
    /// 新订单
    New = 1,
    /// 全部完成
    Filled = 3,
    /// 部分完成
    PartiallyFilled = 2,
    /// 已取消
    Canceled = 4,
    /// 部分完成且已取消
    PartiallyCanceled = 5,
    /// 已过期
    Expired,
}
impl OrderStatus {
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Canceled | Self::PartiallyCanceled | Self::Expired | Self::Filled)
    }
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
