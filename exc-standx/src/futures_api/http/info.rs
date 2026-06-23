use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

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
    pub min_order_qty: f64,
    pub price_tick_decimals: i8,
    pub qty_tick_decimals: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee: f64,
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
        "/api/query_symbol_info".to_string()
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
    pub index_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub next_funding_time: OffsetDateTime,
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
        "/api/query_symbol_market".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub time: OffsetDateTime,
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
        "/api/query_funding_rates".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
