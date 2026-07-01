use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub market: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_order_size_change: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_price_change: f64,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L2Config {
    pub synthetic_id: String,
    pub synthetic_resolution: f64,
    pub collateral_id: String,
    pub collateral_resolution: f64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub name: String,
    pub status: String,
    pub trading_config: TradingConfig,
    pub l2_config: L2Config,
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
        "/api/v1/info/markets".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    #[serde(skip)]
    pub market: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub index_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    pub next_funding_rate: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/api/v1/info/markets/{}/stats", self.market)
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    #[serde(skip)]
    pub market: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub f: f64,
    #[serde(rename = "T")]
    pub t: u64,
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
        format!("/api/v1/info/{}/funding", self.market)
    }
    fn need_sign(&self) -> bool {
        false
    }
}
