use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBalanceRequest {
    #[serde(skip)]
    pub account_id: u64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Asset {
    pub currency: String,
    pub r#type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Assets {
    pub list: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetBalanceResponse {
    pub data: Assets,
}

impl Rest for GetBalanceRequest {
    type Response = GetBalanceResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::SpotApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("/v1/account/accounts/{}/balance", self.account_id)
    }
    fn need_sign(&self) -> bool {
        true
    }
}
