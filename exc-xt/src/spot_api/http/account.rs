use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnRequest {
    pub scope: &'static str,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub total_assets: f64,
}

impl Rest for GetEarnRequest {
    type Response = GetEarnResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v4/finance/balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
