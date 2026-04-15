use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, VecSkipError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde_as(as = "DisplayFromStr")]
        tick_size: f64,
    },
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde_as(as = "DisplayFromStr")]
        step_size: f64,
    },
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    #[serde_as(as = "VecSkipError<_>")]
    pub filters: Vec<Filter>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub symbols: Vec<SymbolInfo>,
}

impl Rest for GetInfoRequest {
    type Response = GetInfoResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v3/exchangeInfo".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
