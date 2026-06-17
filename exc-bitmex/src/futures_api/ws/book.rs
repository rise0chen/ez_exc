use crate::futures_api::types::OrderSide;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Partial,
    Update,
    Insert,
    Delete,
}
impl Action {
    pub fn is_init(&self) -> bool {
        matches!(self, Self::Partial)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub size: Option<i64>,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersResponse {
    pub action: Action,
    pub data: Vec<Order>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    pub symbol: String,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub action: Action,
    pub data: Vec<Depth>,
}
