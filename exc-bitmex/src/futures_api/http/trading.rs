use crate::futures_api::types::{OrderSide, OrderStatus, PositionSide, TimeInForce, serialize_filter};
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub cl_ord_i_d: Option<String>,
    pub side: OrderSide,
    pub strategy: PositionSide,
    pub display_qty: Decimal,
    pub order_qty: Decimal,
    pub price: Decimal,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_i_d: String,
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
        "/api/v1/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub order_i_d: Vec<String>,
    pub cl_ord_i_d: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_i_d: String,
    pub cl_ord_i_d: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = Vec<CancelOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/api/v1/order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub symbol: String,
    #[serde(serialize_with = "serialize_filter")]
    pub filter: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub order_i_d: String,
    pub cl_ord_i_d: Option<String>,
    pub side: OrderSide,
    pub strategy: PositionSide,
    pub order_qty: i64,
    pub cum_qty: Option<i64>,
    pub avg_px: Option<f64>,
    pub ord_status: OrderStatus,
}

impl Rest for GetOrderRequest {
    type Response = Vec<GetOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
