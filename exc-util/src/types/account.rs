#[derive(Debug, Clone, Default)]
pub struct Position {
    pub id: String,
    /// 持仓数量
    pub size: f64,
    /// 开仓均价
    pub price: f64,
}
impl Position {
    pub fn new(size: f64) -> Self {
        Self {
            id: String::new(),
            size,
            price: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Balance {
    /// 现货/资金 账户
    pub spot: f64,
    /// 合约/统一 账户
    pub future: f64,
    /// 理财 账户
    pub finance: f64,
    /// 总资产
    pub total: f64,
}
impl Balance {
    pub fn new(spot: f64, future: f64, finance: f64) -> Self {
        Self {
            spot,
            future,
            finance,
            total: spot + future + finance,
        }
    }
}
