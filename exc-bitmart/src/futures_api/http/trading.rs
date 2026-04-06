use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    pub symbol: String,
    pub order_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub order_id: String,
    pub client_order_id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub deal_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub deal_avg_price: f64,
    pub side: OrderSide,
    pub state: OrderStatus,
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
        "/contract/private/order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersRequest {
    pub symbol: String,
}

impl Rest for GetOpenOrdersRequest {
    type Response = Vec<GetOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/get-open-orders".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCloseOrdersRequest {
    pub symbol: String,
    pub client_order_id: Option<String>,
}

impl Rest for GetCloseOrdersRequest {
    type Response = Vec<GetOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/order-history".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTradesRequest {
    pub symbol: String,
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTradesResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub paid_fees: f64,
}

impl Rest for GetTradesRequest {
    type Response = Vec<GetTradesResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/trades".to_string()
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
    pub client_order_id: Option<String>,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub leverage: Decimal,
    pub mode: TimeInForce,
    pub size: u64,
    pub price: Decimal,
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
        "/contract/private/submit-order".to_string()
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
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
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
        "/contract/private/cancel-order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
