use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub symbol_id: String,
    pub client_order_id: Option<String>,
    pub side: OrderSide,
    pub order_side: crate::futures_api::types::OrderSide,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_base: Option<Decimal>,
    pub amount: Decimal,
    pub price: Decimal,
    pub price_type: &'static str,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: String,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/capi/v1/futures/order/create".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    pub orig_client_order_id: Option<String>,
    pub order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/capi/v1/futures/order/cancel".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
