use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::types::order::{FuturesOpenType, OrderSide, OrderType};
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
    #[serde_as(as = "DisplayFromStr")]
    pub vol: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub leverage: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
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
pub struct CancelOrderResponse;

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

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
