use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// price, size
pub struct Order {
    #[serde_as(as = "DisplayFromStr")]
    pub p: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub m: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub t: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub biz: String,
    pub pair_code: String,
    pub data: Depth,
}
