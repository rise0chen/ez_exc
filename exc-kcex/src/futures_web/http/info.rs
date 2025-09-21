use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    #[serde(skip)]
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    pub funding_rate: f64,
    pub next_settle_time: u64,
    pub collect_cycle: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/fapi/v1/contract/funding_rate/{}", self.symbol)
    }
    fn need_sign(&self) -> bool {
        false
    }
}
