use super::super::types::{OrderSide, OrderStatus, OrderType, TimeInForce};
use crate::response::List;
use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::symbol::SymbolKind;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: SymbolKind,
    pub symbol: String,
    pub is_leverage: u8,
    /// buy：买， sell：卖
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub qty: Decimal,
    pub market_unit: String,
    pub price: Decimal,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub order_link_id: Option<String>,
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
        "/v5/order/create".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: SymbolKind,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub qty: Option<f64>,
    pub price: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub order_id: String,
    pub order_link_id: Option<String>,
}

impl Rest for AmendOrderRequest {
    type Response = AmendOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/v5/order/amend".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: SymbolKind,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_id: String,
    pub order_link_id: Option<String>,
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
        "/v5/order/cancel".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub category: SymbolKind,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: Option<String>,
    /// buy：买， sell：卖
    pub side: OrderSide,
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    pub market_unit: String,
    pub price: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    pub cum_exec_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cum_exec_value: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cum_exec_fee: f64,
    pub order_status: OrderStatus,
}

impl Rest for GetOrderRequest {
    type Response = List<GetOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v5/order/realtime".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
