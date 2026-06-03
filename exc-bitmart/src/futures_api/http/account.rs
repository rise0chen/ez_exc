use super::super::types::*;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceResponse {
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub equity: f64,
}

impl Rest for GetBalanceRequest {
    type Response = Vec<GetBalanceResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/assets-detail".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEarnRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub total_user_sum: f64,
}

impl Rest for GetEarnRequest {
    type Response = GetEarnResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://api-cloud.bitmart.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/newearn/cloud/v1/earn".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub position_side: PositionSide,
    #[serde_as(as = "DisplayFromStr")]
    pub position_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub position_value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionResponse(pub Vec<Asset>);

impl Rest for GetPositionRequest {
    type Response = GetPositionResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/position-v2".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFeeRequest {
    pub symbol: String,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFeeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate: f64,
}

impl Rest for GetFeeRequest {
    type Response = GetFeeResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/contract/private/trade-fee-rate".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
