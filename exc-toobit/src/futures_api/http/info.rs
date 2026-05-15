use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, Map, VecSkipError, serde_as};

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
        min_qty: f64,
        #[serde_as(as = "DisplayFromStr")]
        step_size: f64,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde_as(as = "DisplayFromStr")]
        min_notional: f64,
    },
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    pub status: String,
    #[serde_as(as = "DisplayFromStr")]
    pub contract_multiplier: f64,
    #[serde_as(as = "VecSkipError<_>")]
    pub filters: Vec<Filter>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub contracts: Vec<SymbolInfo>,
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
        "/api/v1/exchangeInfo".to_string()
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

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    #[serde_as(as = "Map<_, DisplayFromStr>")]
    pub index: Vec<(String, f64)>,
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
        "/quote/v1/index".to_string()
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
    pub rate: f64,
    pub period: String,
    #[serde_as(as = "DisplayFromStr")]
    pub next_funding_time: u64,
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
        "/api/v1/futures/fundingRate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    pub limit: u8,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub settle_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub settle_time: u64,
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
        "/api/v1/futures/historyFundingRate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
