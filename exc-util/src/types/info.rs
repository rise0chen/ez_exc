#[derive(Debug, Default, Clone)]
pub struct FundingRate {
    pub rate: f64,
    pub time: u64,
    pub interval: u64,
}
