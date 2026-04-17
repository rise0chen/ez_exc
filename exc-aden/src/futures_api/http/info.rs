use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoRequest {
    pub contract: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub quanto_multiplier: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub order_price_round: f64,
    pub order_size_min: f64,
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
        format!("/api/v1/dex_futures/usdt/contracts/{}", self.contract)
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateRequest {
    #[serde(skip)]
    pub contract: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    pub funding_next_apply: u64,
    pub funding_interval: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/api/v1/dex_futures/usdt/contracts/{}", self.contract)
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryRequest {
    pub contract: String,
    pub from: Option<u64>,
    pub to: Option<u64>,
    pub limit: Option<u8>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub r: f64,
    pub t: u64,
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
        "/api/v1/dex_futures/usdt/funding_rate".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
