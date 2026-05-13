use crate::futures_api::types::{Id, PositionSide};
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub available_margin: f64,
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
        "/v1/perpum/account/getUserAssets".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub instrument: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    pub id: Id,
    pub direction: PositionSide,
    pub current_piece: f64,
    pub open_price: Option<f64>,
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
        "/v1/perpum/positions".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
