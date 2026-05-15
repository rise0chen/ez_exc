use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    pub symbol: String,
    pub limit: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size
pub struct Order(#[serde_as(as = "DisplayFromStr")] pub f64, #[serde_as(as = "DisplayFromStr")] pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub a: Vec<Order>,
    pub b: Vec<Order>,
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
        "/quote/v1/depth".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
