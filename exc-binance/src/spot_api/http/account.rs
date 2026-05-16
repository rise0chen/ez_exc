use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub asset: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub asset: String,
    #[serde_as(as = "DisplayFromStr")]
    pub cross_margin_asset: f64,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://papi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/papi/v1/balance".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    #[serde_as(as = "DisplayFromStr")]
    pub taker: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maker: f64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeResponse {
    pub standard_commission: FeeRate,
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
        "/api/v3/account/commission".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
