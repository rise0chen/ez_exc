use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetDepthRequest {
    pub symbol: String,
    pub depth: u16,
    pub r#type: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// price, size
pub struct Order(pub f64, pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OrderList {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub version: u64,
    pub ts: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetDepthResponse {
    pub tick: OrderList,
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
        "/market/depth".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
