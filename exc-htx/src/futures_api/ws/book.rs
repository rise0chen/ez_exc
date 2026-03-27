use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order(pub f64, pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Depth {
    pub asks: Vec<Order>,
    pub bids: Vec<Order>,
    pub ts: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepthResponse {
    pub tick: Depth,
}
