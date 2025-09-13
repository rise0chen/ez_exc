use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStRateRequest {
    pub project_no: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStRateResponse {
    ///转换比例
    #[serde_as(as = "DisplayFromStr")]
    pub convert_rate: f64,
    // 赎回期
    pub redeem_period: f64,
    ///年化利率
    #[serde_as(as = "DisplayFromStr")]
    pub apy: f64,
    ///上次结算时间
    pub last_epoch_update_time: u64,
    ///下次结算时间
    pub epoch_next_update_time: u64,
    ///赎回费率
    #[serde_as(as = "DisplayFromStr")]
    pub fee: f64,
}

impl Rest for GetStRateRequest {
    type Response = GetStRateResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotWeb
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/financialactivity/blc/pledge/project/detail".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
