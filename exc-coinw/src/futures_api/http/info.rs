use exc_util::interface::{ApiKind, Method, Rest};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::PrimitiveDateTime;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub name: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub one_lot_size: f64,
    pub min_size: f64,
    pub price_precision: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee: f64,
    pub settled_at: u64,
    pub settlement_rate: f64,
    pub settled_period: u64,
}

impl Rest for GetInfoRequest {
    type Response = Vec<GetInfoResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v1/perpum/instruments".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickerRequest {
    pub instrument: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTickerResponse {
    pub fair_price: f64,
}

impl Rest for GetTickerRequest {
    type Response = Vec<GetTickerResponse>;

    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "/v1/perpumPublic/ticker".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    pub instrument: String,
    pub day: u8,
}

time::serde::format_description!(datetime_format, PrimitiveDateTime, "[year]-[month]-[day] [hour]:[minute]");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    #[serde(with = "datetime_format")]
    pub created_date: PrimitiveDateTime,
    pub funding_rate: f64,
}

impl Rest for GetFundingRateHistoryRequest {
    type Response = Vec<GetFundingRateHistoryResponse>;

    fn host(&self) -> Option<&'static str> {
        Some("https://futuresapi.coinw.com")
    }
    fn api_kind(&self) -> ApiKind {
        ApiKind::FuturesApi
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/v1/futuresc/public/selectFundingRateHistory".into()
    }
    fn need_sign(&self) -> bool {
        false
    }
}
