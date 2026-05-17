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
    pub cs: f64,
    pub pu: f64,
    pub min_v: f64,
    pub tfr: f64,
}

impl Rest for GetInfoRequest {
    type Response = Vec<GetInfoResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/contract/detailV2".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceRequest {
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    pub index_price: f64,
}

impl Rest for GetIndexPriceRequest {
    type Response = GetIndexPriceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/contract/ticker".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    #[serde(skip)]
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    pub funding_rate: f64,
    pub next_settle_time: u64,
    pub collect_cycle: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/fapi/v1/contract/funding_rate/{}", self.symbol)
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    pub page_size: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    pub funding_rate: f64,
    pub settle_time: u64,
    pub collect_cycle: u64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    pub result_list: Vec<FundingRateHistory>,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = GetFundingRateHistoryResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/contract/funding_rate/history".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
