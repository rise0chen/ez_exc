use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::types::order::OrderSide;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub symbol: String,
    pub orig_client_order_id: Option<String>,
    pub order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub avg_price: Option<f64>,
    pub side: OrderSide,
    pub status: OrderStatus,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub new_client_order_id: Option<String>,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    pub quantity: Decimal,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        "/fapi/v1/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub orig_client_order_id: Option<String>,
    pub order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_id: u64,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/fapi/v1/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
