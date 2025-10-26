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
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
}

impl Rest for GetFundingRateRequest {
    type Response = GetFundingRateResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://fapi.binance.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/fapi/v1/premiumIndex".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
