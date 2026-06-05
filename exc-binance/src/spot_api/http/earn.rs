use crate::response::List;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWbethRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWbethResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub annual_percentage_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub exchange_rate: f64,
    pub time: u64,
}

impl Rest for GetWbethRequest {
    type Response = List<GetWbethResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/sapi/v1/eth-staking/eth/history/rateHistory".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBnsolRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBnsolResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub annual_percentage_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub exchange_rate: f64,
    pub time: u64,
}

impl Rest for GetBnsolRequest {
    type Response = List<GetBnsolResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/sapi/v1/sol-staking/sol/history/rateHistory".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
