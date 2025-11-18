use super::super::types::{OrderStatus, OrderType};
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlaceOrderRequest {
    pub account_id: u64,
    pub symbol: String,
    pub client_order_id: Option<String>,
    pub source: &'static str,
    pub r#type: OrderType,
    pub amount: Decimal,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlaceOrderResponse {
    pub data: String,
}

impl Rest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/v1/order/orders/place".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CancelOrderRequest {
    #[serde(skip)]
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CancelOrderResponse {
    pub data: String,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/v1/order/orders/{}/submitcancel", id);
        }
        if self.client_order_id.is_some() {
            return "/v1/order/orders/getClientOrder".into();
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    #[serde(skip)]
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OrderDetail {
    pub id: u64,
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub r#type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub field_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub field_cash_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub field_fees: f64,
    pub state: OrderStatus,
}
#[derive(Debug, Deserialize)]
pub struct GetOrderResponse {
    pub data: OrderDetail,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/v1/order/orders/{}", id);
        }
        if self.client_order_id.is_some() {
            return "/v1/order/orders/getClientOrder".into();
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
