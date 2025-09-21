use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    #[serde(skip)]
    pub symbol: String,
    pub limit: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size, order_num
pub struct Order(pub f64, pub f64, pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub version: u64,
    pub timestamp: u64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/fapi/v1/contract/depth_step/{}", self.symbol)
    }
    fn need_sign(&self) -> bool {
        false
    }
}
