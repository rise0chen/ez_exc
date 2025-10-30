use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::types::order::{FuturesOpenType, OrderSide, OrderStatus, OrderType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub external_oid: Option<String>,
    #[serde_as(as = "FromInto<i8>")]
    pub side: OrderSide,
    #[serde_as(as = "FromInto<i8>")]
    pub open_type: FuturesOpenType,
    #[serde_as(as = "FromInto<i8>")]
    pub r#type: OrderType,
    pub vol: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    pub leverage: f64,
    pub price: Decimal,
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
        "/api/v1/private/order/create".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest(pub Vec<String>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {}

impl Rest for CancelOrderRequest {
    type Response = Vec<CancelOrderResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/api/v1/private/order/cancel".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    #[serde(skip)]
    pub symbol: String,
    #[serde(skip)]
    pub order_id: Option<String>,
    #[serde(skip)]
    pub external_oid: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    pub symbol: String,
    pub order_id: String,
    pub external_oid: Option<String>,
    pub price: f64,
    pub vol: f64,
    pub deal_vol: f64,
    pub deal_avg_price: f64,
    pub taker_fee: f64,
    pub maker_fee: f64,
    #[serde_as(as = "FromInto<i8>")]
    pub state: OrderStatus,
    #[serde_as(as = "FromInto<i8>")]
    pub order_type: OrderType,
    #[serde_as(as = "FromInto<i8>")]
    pub side: OrderSide,
}

impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/api/v1/private/order/get/{}", id);
        }
        if let Some(id) = &self.external_oid {
            return format!("/api/v1/private/order/external/{}/{}", self.symbol, id);
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
