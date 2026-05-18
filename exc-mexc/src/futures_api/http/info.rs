use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub contract_size: f64,
    pub price_scale: i8,
    pub vol_scale: i8,
    pub price_unit: f64,
    pub vol_unit: f64,
    pub min_vol: f64,
    pub taker_fee_rate: f64,
    pub state: i8,
}

impl Rest for GetInfoRequest {
    type Response = GetInfoResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/contract/detail/country".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
