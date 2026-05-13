use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    pub base: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size
pub struct Order {
    pub p: f64,
    pub m: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub t: u64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v1/perpumPublic/depth".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
