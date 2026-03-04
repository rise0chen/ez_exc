use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthRequest {
    pub contract_code: String,
    pub r#type: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order(pub f64, pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Depth {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub ts: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthResponse {
    pub tick: Depth,
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
        "/linear-swap-ex/market/depth".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
