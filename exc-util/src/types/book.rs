#[derive(Debug)]
pub struct Order {
    pub price: f64,
    pub size: f64,
}
impl Order {
    pub fn new(price: f64, size: f64) -> Self {
        Self { price, size }
    }
}

#[derive(Debug)]
pub struct Depth {
    pub bid: Vec<Order>,
    pub ask: Vec<Order>,
    pub price: f64,
    pub version: u64,
}
