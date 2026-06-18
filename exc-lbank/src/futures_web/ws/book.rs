use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Order(#[serde_as(as = "DisplayFromStr")] pub f64, #[serde_as(as = "DisplayFromStr")] pub f64);

#[derive(Debug, Deserialize)]
pub struct GetDepthResponse {
    pub b: Vec<Order>,
    pub s: Vec<Order>,
}
