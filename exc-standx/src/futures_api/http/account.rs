use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
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
        "/api/query_balance".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub entry_price: f64,
}

impl Rest for GetPositionRequest {
    type Response = Vec<GetPositionResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/query_positions".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
