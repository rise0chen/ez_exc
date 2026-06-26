use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GetTickerResponse {
    /// symbol
    pub a: String,
    /// index_price
    #[serde_as(as = "DisplayFromStr")]
    pub d: f64,
}
