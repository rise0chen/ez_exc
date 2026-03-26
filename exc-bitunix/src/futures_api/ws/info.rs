use serde::{Deserialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub ip: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub mp: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub fr: f64,
}