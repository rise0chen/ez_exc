use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate_interval: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub next_update: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = Vec<GetFundingRateResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/market/current-fund-rate".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
