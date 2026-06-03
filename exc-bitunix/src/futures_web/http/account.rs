use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub coin: &'static str,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub total: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub spot_total: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub futures_total: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub earn_total: f64,
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
        "/web/finance/coin/balance/total".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
