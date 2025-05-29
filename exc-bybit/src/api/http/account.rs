use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub account_type: &'static str,
    pub coin: &'static str,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde_as(as = "DisplayFromStr")]
    pub wallet_balance: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub balance: Balance,
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
        "/v5/asset/transfer/query-account-coin-balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
