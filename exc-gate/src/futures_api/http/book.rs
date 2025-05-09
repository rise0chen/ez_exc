use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthRequest {
    pub contract: String,
    pub limit: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order {
    #[serde_as(as = "DisplayFromStr")]
    pub p: f64,
    pub s: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthResponse {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub update: f64,
    pub current: f64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn host(&self) -> Option<&'static str> {
        Some("https://www.gate.io")
    }
    fn path(&self) -> String {
        "/apiw/v2/futures/usdt/order_book".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
