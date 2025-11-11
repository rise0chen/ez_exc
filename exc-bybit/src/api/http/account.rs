use super::super::types::OrderSide;
use exc_util::interface::{ApiKind, Method, Rest};
use exc_util::symbol::SymbolKind;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub account_type: &'static str,
    pub coin: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub coin: String,
    #[serde_as(as = "DisplayFromStr")]
    pub equity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub locked: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde_as(as = "DisplayFromStr")]
    pub total_available_balance: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub total_margin_balance: f64,
    pub coin: Vec<Asset>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub list: Vec<Balance>,
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
        "/v5/account/wallet-balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub category: SymbolKind,
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    pub side: OrderSide,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    pub list: Vec<Position>,
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
        "/v5/position/list".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
