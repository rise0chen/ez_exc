use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub position_margin: f64,
    pub available_balance: f64,
    pub equity: f64,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/private/account/asset/USDT".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
