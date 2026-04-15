use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountRequest {
    pub by: &'static str,
    pub value: i64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub asset_id: u32,
    pub symbol: String,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
}
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Position {
    pub market_id: i16,
    pub sign: i16,
    #[serde_as(as = "DisplayFromStr")]
    pub position: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub avg_entry_price: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Account {
    #[serde_as(as = "DisplayFromStr")]
    pub cross_asset_value: f64,
    pub assets:Vec<Asset>,
    pub positions: Vec<Position>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountResponse {
    pub accounts: Vec<Account>,
}

impl Rest for GetAccountRequest {
    type Response = GetAccountResponse;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/api/v1/account".to_string()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
