use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnCoin {
    pub subscription_coin: String,
    #[serde_as(as = "DisplayFromStr")]
    pub exchange_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fee_rate: f64,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnResponse {
    pub coin: String,
    #[serde_as(as = "DisplayFromStr")]
    pub max_apr: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub min_apr: f64,
    pub subscription_coin_list: Vec<EarnCoin>,
}

impl Rest for GetEarnRequest {
    type Response = Vec<GetEarnResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/earn/elite-product".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
