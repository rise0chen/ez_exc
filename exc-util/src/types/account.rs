#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    /// 持仓数量
    pub size: f64,
    /// 开仓均价
    pub price: f64,
}
impl Position {
    pub fn new(size: f64) -> Self {
        Self { size, price: 0.0 }
    }
}
