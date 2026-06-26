use crate::response::List;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub product_group: &'static str,
    pub instrument: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub price_tick: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume_tick: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_cost: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_volume: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub force_close_fee_rate: f64,
}

impl Rest for GetInfoRequest {
    type Response = GetInfoResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/instrment/v1/tradeRuleDetail".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    pub asset: &'static str,
    pub product_group: &'static str,
    pub instrument_i_d: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    pub funding_rate_timestamp: u64,
    pub next_funding_rate_timestamp: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/cfd/agg/v1/sendQryAll".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub product_group: &'static str,
    pub instrument_i_d: String,
    pub page_size: u8,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    pub funding_interval_hours: u64,
    pub funding_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = List<GetFundingRateHistoryResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/instrment/v1/fundRateList".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
