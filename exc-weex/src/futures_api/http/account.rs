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
    pub available: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub frozen: f64,
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
        "/capi/v2/account/assets".to_string()
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
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    pub side: OrderSide,
    #[serde_as(as = "DisplayFromStr")]
    pub open_value: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub unrealize_pnl: f64,
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
        "/capi/v2/account/position/singlePosition".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
