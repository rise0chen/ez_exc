use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
/// price, size
pub struct Order(#[serde_as(as = "DisplayFromStr")] pub f64, #[serde_as(as = "DisplayFromStr")] pub f64);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepthResponse {
    pub a: Vec<Order>,
    pub b: Vec<Order>,
}
