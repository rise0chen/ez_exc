use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    pub cl_ord_id: Option<String>,
    pub order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub id: u64,
    pub cl_ord_id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fill_qty: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub fill_avg_price: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
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
        "/api/query_order".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub cl_ord_id: Option<String>,
    pub side: OrderSide,
    pub reduce_only: bool,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub qty: Decimal,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderResponse {}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/new_order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    pub cl_ord_id: Option<String>,
    pub order_id: Option<String>,
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
        "/api/cancel_order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
