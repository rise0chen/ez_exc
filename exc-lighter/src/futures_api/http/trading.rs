use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::futures_api::types::OrderStatus;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order {
    pub order_index: i64,
    pub client_order_id: String,
    pub market_index: i16,
    pub is_ask: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub initial_base_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_base_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_quote_amount: f64,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderRequest {
    pub auth: String,
    pub account_index: i64,
    pub market_id: i16,
    pub limit: u16,
    #[serde(skip)]
    pub active: bool,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderResponse {
    pub orders: Vec<Order>,
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
        if self.active {
            "/api/v1/accountActiveOrders".into()
        } else {
            "/api/v1/accountInactiveOrders".into()
        }
    }
    fn need_sign(&self) -> bool {
        false
    }
}
