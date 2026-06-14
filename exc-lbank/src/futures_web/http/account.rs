use crate::response::Data;
use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub __auto_login: bool,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assert {
    #[serde_as(as = "DisplayFromStr")]
    pub to_usd: f64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub spot_asset: Assert,
    pub futures_asset_covert: Assert,
    pub investment_asset: Assert,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn host(&self) -> Option<&'static str> {
        Some("https://www.lbank.com/lbk-api")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/coin-wallet-center/customer/asset/overview".to_string()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionRequest {
    pub product_group: &'static str,
    pub instrument: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetPositionResponse {
    pub position_i_d: String,
    #[serde_as(as = "DisplayFromStr")]
    pub posi_direction: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub position: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub position_cost: f64,
}

impl Rest for GetPositionRequest {
    type Response = Data<Vec<GetPositionResponse>>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/query/v1.0/Position".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRequest {
    pub product_group: &'static str,
    pub instrument_i_d: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub maker_open_fee_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_open_fee_rate: f64,
}

impl Rest for GetFeeRequest {
    type Response = GetFeeResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/cfd/user/v1/userFee".into()
    }
    fn need_sign(&self) -> bool {
        true
    }
}
