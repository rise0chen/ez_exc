use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthRequest {
    pub currency_pair: String,
    pub limit: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order(#[serde_as(as = "DisplayFromStr")] pub f64, #[serde_as(as = "DisplayFromStr")] pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthResponse {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub update: u64,
    pub current: u64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v4/spot/order_book".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
