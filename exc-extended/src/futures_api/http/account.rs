use crate::futures_api::types::PositionSide;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub equity: f64,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/user/balance".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpotBalanceRequest {}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpotBalanceResponse {
    pub asset: String,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub average_entry_price: Option<f64>,
    #[serde_as(as = "DisplayFromStr")]
    pub notional_value: f64,
}

impl Rest for GetSpotBalanceRequest {
    type Response = Vec<GetSpotBalanceResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/user/spot/balances".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub market: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionResponse {
    pub id: u64,
    pub side: PositionSide,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub open_price: f64,
}

impl Rest for GetPositionRequest {
    type Response = Vec<GetPositionResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/user/positions".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRequest {
    pub market: String,
    pub builder_id: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub builder_fee_rate: f64,
}

impl Rest for GetFeeRequest {
    type Response = Vec<GetFeeResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::Common
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/user/fees".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
