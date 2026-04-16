use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoRequest {
    #[serde(skip)]
    pub currency_pair: String,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoResponse {
    pub amount_precision: i8,
    pub precision: i8,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub min_base_amount: Option<f64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub min_quote_amount: Option<f64>,
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
        format!("/api/v4/spot/currency_pairs/{}", self.currency_pair)
    }
    fn need_sign(&self) -> bool {
        false
    }
}
