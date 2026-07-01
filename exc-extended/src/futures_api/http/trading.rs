use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, DisplayFromStr, OneOrMany};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub r: String,
    pub s: String,
}
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settlement {
    pub signature: Signature,
    pub stark_key: String,
    pub collateral_position: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: u64,
    pub external_id: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub price: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub average_price: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_qty: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub payed_fee: Option<f64>,
    pub side: OrderSide,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    #[serde(skip)]
    pub external_id: Option<String>,
    #[serde(skip)]
    pub order_id: Option<String>,
}
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GetOrderResponse(#[serde_as(as = "OneOrMany<_, PreferOne>")] pub Vec<Order>);
impl Rest for GetOrderRequest {
    type Response = GetOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            format!("/api/v1/user/orders/{}", id)
        } else if let Some(id) = &self.external_id {
            format!("/api/v1/user/orders/external/{}", id)
        } else {
            String::new()
        }
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub market: String,
    pub id: Option<String>,
    pub side: OrderSide,
    pub qty: Decimal,
    pub price: Decimal,
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
    pub reduce_only: bool,
    pub post_only: bool,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    pub expiry_epoch_millis: u64,
    pub settlement: Settlement,
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: u32,
    pub self_trade_protection_level: &'static str,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_fee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub id: u64,
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
        "/api/v1/user/order".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip)]
    pub order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        if self.external_id.is_some() {
            "/api/v1/user/order".to_string()
        } else if let Some(id) = &self.order_id {
            format!("/api/v1/user/order/{}", id)
        } else {
            String::new()
        }
    }
    fn need_sign(&self) -> bool {
        true
    }
}
