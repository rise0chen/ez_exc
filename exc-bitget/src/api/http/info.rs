use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub category: &'static str,
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub price_precision: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub quantity_precision: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_qty: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate: f64,
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
        "/api/v3/market/instruments".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceRequest {
    pub symbol: String,
    pub product_type: &'static str,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,
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
        "/api/v2/mix/market/symbol-price".to_string()
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
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate_interval: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub next_update: u64,
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
        "/api/v3/market/current-fund-rate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub category: &'static str,
    pub symbol: String,
    pub limit: u8,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate_timestamp: u64,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    pub result_list: Vec<FundingRateHistory>,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = GetFundingRateHistoryResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/market/history-fund-rate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
