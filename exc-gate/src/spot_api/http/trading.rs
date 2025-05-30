use super::super::types::{OrderSide, OrderStatus, OrderType, TimeInForce};
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderRequest {
    pub currency_pair: String,
    pub text: Option<String>,
    pub r#type: OrderType,
    pub time_in_force: TimeInForce,
    pub side: OrderSide,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrderResponse {
    pub id: String,
    pub text: Option<String>,
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
        "/api/v4/spot/orders".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AmendOrderRequest {
    #[serde(skip)]
    pub order_id: Option<String>,
    #[serde(skip)]
    pub text: Option<String>,
    pub currency_pair: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub amount: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub price: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AmendOrderResponse {
    pub id: String,
    pub text: Option<String>,
}

impl Rest for AmendOrderRequest {
    type Response = AmendOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/api/v4/spot/orders/{}", id);
        }
        if let Some(id) = &self.text {
            return format!("/api/v4/spot/orders/{}", id);
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    #[serde(skip)]
    pub order_id: Option<String>,
    #[serde(skip)]
    pub text: Option<String>,
    pub currency_pair: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderResponse {
    pub id: String,
    pub text: Option<String>,
}

impl Rest for CancelOrderRequest {
    type Response = CancelOrderResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        if let Some(id) = &self.order_id {
            return format!("/api/v4/spot/orders/{}", id);
        }
        if let Some(id) = &self.text {
            return format!("/api/v4/spot/orders/{}", id);
        }
        String::new()
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
    pub text: Option<String>,
    pub currency_pair: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub id: String,
    pub text: Option<String>,
    pub currency_pair: String,
    pub side: OrderSide,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_total: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub gt_maker_fee: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub gt_taker_fee: Option<f64>,
    pub status: OrderStatus,
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
            return format!("/api/v4/spot/orders/{}", id);
        }
        if let Some(id) = &self.text {
            return format!("/api/v4/spot/orders/{}", id);
        }
        String::new()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
