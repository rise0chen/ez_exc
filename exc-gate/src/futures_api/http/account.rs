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
    pub total_margin_balance: f64,
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
        "/api/v4/unified/accounts".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
