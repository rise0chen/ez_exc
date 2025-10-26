use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

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
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub last_update_id: u64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://fapi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/depth".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
