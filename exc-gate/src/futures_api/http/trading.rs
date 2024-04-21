use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as,DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub contract: String,
    pub text: Option<String>,
    pub size: i64,
    pub price: f64,
    pub tif: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderResponse {
    pub id: i64,
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
        "/api/v4/futures/usdt/orders".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    #[serde(skip)]
    pub order_id: Option<String>,
    #[serde(skip)]
    pub external_oid: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub contract: String,
    pub id: i64,
    pub text: Option<String>,
    pub size: f64,
    pub left: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fill_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub tkfr: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub mkfr: f64,
    pub finish_as: String,
    pub status: String,
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
        if let Some(id) = &self.order_id {
            return format!("/api/v4/futures/usdt/orders/{}", id);
        }
        if let Some(id) = &self.external_oid {
            return format!("/api/v4/futures/usdt/orders/{}", id);
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
