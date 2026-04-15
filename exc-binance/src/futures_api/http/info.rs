use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, VecSkipError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde_as(as = "DisplayFromStr")]
        tick_size: f64,
    },
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde_as(as = "DisplayFromStr")]
        step_size: f64,
    },
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    #[serde_as(as = "VecSkipError<_>")]
    pub filters: Vec<Filter>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub symbols: Vec<SymbolInfo>,
}

impl Rest for GetInfoRequest {
    type Response = GetInfoResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://fapi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/exchangeInfo".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://fapi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/premiumIndex".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u8>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    pub funding_time: u64,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = Vec<GetFundingRateHistoryResponse>;

    fn host(&self) -> Option<&'static str> {
        Some("https://fapi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/fundingRate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
