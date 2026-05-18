use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub symbol: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    ///交易对ID
    pub id: String,
    ///quotebase币ID
    pub mcd: String,
    ///base币ID
    pub cd: String,
    ///base币简称
    pub vn: String,
    ///quote币简称
    pub mn: String,
    /// 最小quote数量
    #[serde_as(as = "DisplayFromStr")]
    pub mi: f64,
    ///价格精度
    pub ps: i8,
    ///数量精度
    pub qs: i8,
    /// 吃单手续费
    #[serde_as(as = "DisplayFromStr")]
    pub tfr: f64,
}

impl Rest for GetInfoRequest {
    type Response = GetInfoResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/platform/spot/market-v2/web/symbol/trade".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
