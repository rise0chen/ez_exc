use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub ccy: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub ccy: String,
    #[serde_as(as = "DisplayFromStr")]
    pub avail_bal: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub cash_bal: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub adj_eq: f64,
    pub details: Vec<Asset>,
}

impl Rest for GetBalanceRequest {
    type Response = Vec<GetBalanceResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/account/balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub inst_id: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub pos: f64,
    pub pos_side: String,
    #[serde_as(as = "DisplayFromStr")]
    pub open_avg_px: f64,
}

impl Rest for GetPositionRequest {
    type Response = Vec<GetPositionResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v5/account/positions".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
