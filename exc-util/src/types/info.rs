#[derive(Debug, Default, Clone)]
pub struct FundingRate {
    // 费率
    pub rate: f64,
    // 下次收费时间
    pub time: u64,
    // 收费周期
    pub interval: u64,
    // 溢价周期
    pub premium_interval: u64,
}
