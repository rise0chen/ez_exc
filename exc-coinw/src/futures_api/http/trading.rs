use crate::futures_api::types::{Id, OrderStatus, PositionSide, TimeInForce};
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, PickFirst};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub instrument: String,
    pub third_order_id: Option<String>,
    pub direction: PositionSide,
    pub leverage: f64,
    pub position_model: u8,
    pub quantity_unit: u8,
    pub quantity: Decimal,
    pub open_price: Decimal,
    pub position_type: TimeInForce,
    pub use_almighty_gold: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub value: Id,
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
        "/v1/perpum/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderRequest {
    pub id: String,
    pub third_order_id: Option<String>,
    pub close_num: Decimal,
    pub order_price: Decimal,
    pub position_type: TimeInForce,
    pub use_almighty_gold: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderResponse {
    pub value: Id,
}

impl Rest for CloseOrderRequest {
    type Response = CloseOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/v1/perpum/positions".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        "/v1/perpum/order".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: Id,
    pub third_order_id: Option<String>,
    pub direction: PositionSide,
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub total_piece: f64,
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    pub trade_piece: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub avg_price: Option<f64>,
    pub order_status: OrderStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersRequest {
    pub instrument: String,
    pub position_type: &'static str,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrderResponse {
    pub rows: Vec<Order>,
}

impl Rest for GetOpenOrdersRequest {
    type Response = GetOpenOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v1/perpum/orders/open".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    pub instrument: String,
    pub origin_type: &'static str,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryResponse {
    pub rows: Vec<Order>,
}

impl Rest for GetOrderHistoryRequest {
    type Response = GetOrderHistoryResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v1/perpum/orders/history".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
