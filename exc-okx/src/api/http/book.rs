use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    pub inst_id: String,
    pub sz: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size, 0, order_num
pub struct Order(
    #[serde_as(as = "DisplayFromStr")] pub f64,
    #[serde_as(as = "DisplayFromStr")] pub f64,
    pub String,
    #[serde_as(as = "DisplayFromStr")] pub f64,
);

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    #[serde_as(as = "DisplayFromStr")]
    pub ts: u64,
}

impl Rest for GetDepthRequest {
    type Response = Vec<GetDepthResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/market/books".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
