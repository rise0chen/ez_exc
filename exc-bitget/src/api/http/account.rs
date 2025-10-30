use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub coin: String,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub account_equity: f64,
    pub assets: Vec<Asset>,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/account/assets".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub category: &'static str,
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde_as(as = "DisplayFromStr")]
    pub total: f64,
    pub pos_side: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    pub list: Option<Vec<Position>>,
}

impl Rest for GetPositionRequest {
    type Response = GetPositionResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/position/current-position".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
