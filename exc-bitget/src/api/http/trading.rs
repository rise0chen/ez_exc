use super::super::types::{OrderSide, OrderStatus, OrderType, TimeInForce};
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: &'static str,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    pub client_oid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: String,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v3/trade/place-order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: &'static str,
    pub order_id: Option<String>,
    pub client_oid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_id: String,
    pub client_oid: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v3/trade/cancel-order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub category: &'static str,
    pub symbol: String,
    pub order_id: Option<String>,
    pub client_oid: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub fee_coin: String,
    pub fee: String,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub client_oid: Option<String>,
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cum_exec_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cum_exec_value: f64,
    pub fee_detail: Vec<Fee>,
    pub order_status: OrderStatus,
    pub side: OrderSide,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/trade/order-info".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
