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
