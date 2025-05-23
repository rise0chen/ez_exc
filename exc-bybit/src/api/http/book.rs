use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::symbol::SymbolKind;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    pub category: SymbolKind,
    pub symbol: String,
    pub limit: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size
pub struct Order(#[serde_as(as = "DisplayFromStr")] pub f64, #[serde_as(as = "DisplayFromStr")] pub f64);

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub a: Vec<Order>,
    pub b: Vec<Order>,
    pub ts: u64,
}

impl Rest for GetDepthRequest {
    type Response = GetDepthResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v5/market/orderbook".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
