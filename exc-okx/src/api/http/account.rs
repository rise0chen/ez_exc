use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub ccy: &'static str,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde_as(as = "DisplayFromStr")]
    pub trading: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub total_bal: f64,
    pub details: Balance,
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
        "/api/v5/asset/asset-valuation".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
