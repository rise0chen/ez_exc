use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStRateRequest {
    pub category: String,
    pub coin: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnInfo {
    ///质押比例
    #[serde_as(as = "DisplayFromStr")]
    pub stake_exchange_rate: f64,
    ///取回比例
    #[serde_as(as = "DisplayFromStr")]
    pub redeem_exchange_rate: f64,
    // 赎回期
    pub redeem_processing_minute: f64,
    ///年化利率
    pub estimate_apr: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStRateResponse {
    pub list: Vec<EarnInfo>,
}

impl Rest for GetStRateRequest {
    type Response = GetStRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v5/earn/product".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
