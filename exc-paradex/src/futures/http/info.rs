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

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFee {
    #[serde_as(as = "DisplayFromStr")]
    pub maker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub spot_maker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub spot_taker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub dated_option_maker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub dated_option_taker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub perp_option_maker_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub perp_option_taker_rate: f64,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfoResponse {
    pub fees: AccountFee,
    pub has_ecosystem_nft: bool,
}
