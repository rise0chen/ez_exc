use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIndexPriceRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SymbolInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIndexPriceResponse {
    pub symbols: Vec<SymbolInfo>,
}

impl Rest for GetIndexPriceRequest {
    type Response = GetIndexPriceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/public/details".to_string()
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
    #[serde_as(as = "DisplayFromStr")]
    pub expected_rate: f64,
    pub funding_time: u64,
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
        "/contract/public/funding-rate".to_string()
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
pub struct FundingRateHistory {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_time: u64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryResponse {
    pub list: Vec<FundingRateHistory>,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = GetFundingRateHistoryResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/public/funding-rate-history".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
