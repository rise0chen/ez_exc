use super::super::types::{OrderType, TimeInForce};
use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::types::order::{OrderSide, OrderStatus};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub new_client_order_id: Option<String>,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    pub quantity: Decimal,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: u64,
    pub client_order_id: Option<String>,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://papi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/papi/v1/margin/order".to_string()
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
    pub order_id: Option<String>,
    pub orig_client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_id: u64,
    pub orig_client_order_id: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://papi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/papi/v1/margin/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub symbol: String,
    pub order_id: Option<String>,
    pub orig_client_order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub order_id: u64,
    pub orig_client_order_id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    #[serde_as(as = "DisplayFromStr")]
    pub orig_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub executed_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://papi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/papi/v1/margin/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
