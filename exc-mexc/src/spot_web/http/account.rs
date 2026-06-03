use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub spot: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub contract: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub financial: f64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub overviews: Vec<Balance>,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/platform/asset/api/asset/overview/convert/v5".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
