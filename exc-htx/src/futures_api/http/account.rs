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
    #[serde_as(as = "DisplayFromStr")]
    pub equity: f64,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v5/account/balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionRequest {
    pub contract_code: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub open_avg_price: f64,
    pub direction: OrderSide,
}

impl Rest for GetPositionRequest {
    type Response = Vec<GetPositionResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v5/trade/position/opens".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
