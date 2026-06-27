use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    pub order_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(alias = "price")]
    #[serde_as(as = "DisplayFromStr")]
    pub initial_price: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub average_filled_price: Option<f64>,
    #[serde(alias = "amount")]
    #[serde_as(as = "DisplayFromStr")]
    pub initial_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_amount: f64,
    pub side: OrderSide,
    pub order_status: Option<OrderStatus>,
    pub created_at: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    pub order_id: String,
}
impl Rest for GetOrderRequest {
    type Response = Vec<Order>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/orders/history_by_id".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOpenOrdersRequest {
    pub account: String,
}
impl Rest for GetOpenOrdersRequest {
    type Response = Vec<Order>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/orders".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderHistoryRequest {
    pub account: String,
    pub limit: u8,
}
impl Rest for GetOrderHistoryRequest {
    type Response = Vec<Order>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/orders/history".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub amount: Decimal,
    pub client_order_id: Option<String>,
    pub price: Decimal,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub symbol: String,
    pub tif: TimeInForce,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderResponse {
    pub order_id: u64,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v1/orders/create".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderResponse {}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v1/orders/cancel".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
