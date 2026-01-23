use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    pub client_id: Option<String>,
    pub order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: String,
    pub client_id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub trade_qty: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub avg_price: Option<f64>,
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
        "/api/v1/futures/trade/get_order_detail".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub client_id: Option<String>,
    pub side: OrderSide,
    pub trade_side: TradeSide,
    pub order_type: OrderType,
    pub effect: TimeInForce,
    pub qty: Decimal,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderResponse {
    pub order_id: String,
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
        "/api/v1/futures/trade/place_order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub order_list: Vec<GetOrderRequest>,
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
        "/api/v1/futures/trade/cancel_orders".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
