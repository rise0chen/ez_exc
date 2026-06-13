use crate::futures_api::types::OrderSide;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthRequest {
    pub symbol: String,
    pub depth: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub side: OrderSide,
    pub size: i64,
    pub price: f64,
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub transact_time: OffsetDateTime,
}

impl Rest for GetDepthRequest {
    type Response = Vec<GetDepthResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/orderBook/L2".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
