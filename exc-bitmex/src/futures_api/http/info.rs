use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub multiplier: i64,
    pub underlying_to_position_multiplier: i64,
    pub quote_to_settle_multiplier: i64,
    pub lot_size: i64,
    pub tick_size: f64,
    pub taker_fee: f64,
    pub maker_fee: f64,

    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub funding_interval: OffsetDateTime,
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub funding_timestamp: OffsetDateTime,
    pub indicative_funding_rate: f64,

    pub indicative_settle_price: f64,
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
        "/api/v1/instrument".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub start_time: OffsetDateTime,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "time::format_description::well_known::Rfc3339")]
    pub timestamp: OffsetDateTime,
    pub funding_rate: f64,
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
        "/api/v1/funding".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
