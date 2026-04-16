use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceRequest {
    pub currency: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Asset {
    pub currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub available: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub locked: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBalanceResponse(pub Vec<Asset>);

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v4/spot/accounts".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFeeRequest {
    pub currency_pair: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GetFeeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub gt_taker_fee: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub futures_taker_fee: f64,
}

impl Rest for GetFeeRequest {
    type Response = GetFeeResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v4/wallet/fee".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
