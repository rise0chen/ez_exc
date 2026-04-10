use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetFundingRateHistoryResponse {
    pub created_at: u64,
    pub funding_index: String,
    pub funding_period_hours: u64,
    pub funding_premium: String,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub funding_rate_8h: f64,
    pub market: String,
}
