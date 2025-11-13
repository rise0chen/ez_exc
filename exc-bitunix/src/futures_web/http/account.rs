use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub assets_balance: f64,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/web/user/vip/user_trade_info".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde_as(as = "DisplayFromStr")]
    pub amount: f64,
    ///  2多 1空
    pub side: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse(pub Vec<Asset>);

impl Rest for GetPositionRequest {
    type Response = GetPositionResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/futures/futures/position/pending".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
