use crate::response::Data;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDepthRequest {
    pub exchange_i_d: &'static str,
    pub product_group: &'static str,
    pub instrument_i_d: String,
    pub depth: u16,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDepthResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub direction: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
}

impl Rest for GetDepthRequest {
    type Response = Vec<Data<GetDepthResponse>>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/cfd/market/v1.0/SendQryMarketOrder".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
