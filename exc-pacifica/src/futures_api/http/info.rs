use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoResponse {
    pub symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    pub tick_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub lot_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub next_funding_rate: f64,
}

impl Rest for GetInfoRequest {
    type Response = Vec<GetInfoResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/info".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateResponse {
    pub symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    pub oracle: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub next_funding: f64,
}

impl Rest for GetFundingRateRequest {
    type Response = Vec<GetFundingRateResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/info/prices".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    pub limit: u8,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    pub created_at: u64,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = Vec<GetFundingRateHistoryResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/funding_rate/history".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
