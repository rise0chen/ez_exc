use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub inst_type: &'static str,
    pub inst_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    #[serde_as(as = "DefaultOnError<Option<DisplayFromStr>>")]
    pub ct_val: Option<f64>,
    #[serde_as(as = "DefaultOnError<Option<DisplayFromStr>>")]
    pub ct_mult: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    pub lot_sz: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub tick_sz: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_sz: f64,
}

impl Rest for GetInfoRequest {
    type Response = Vec<GetInfoResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/public/instruments".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceRequest {
    pub inst_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub idx_px: f64,
}

impl Rest for GetIndexPriceRequest {
    type Response = Vec<GetIndexPriceResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/market/index-tickers".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    pub inst_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub next_funding_time: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = Vec<GetFundingRateResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/public/funding-rate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub inst_id: String,
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub limit: Option<u8>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_time: u64,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = Vec<GetFundingRateHistoryResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/public/funding-rate-history".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
