#[derive(Debug, Clone, Copy)]
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
impl Depth {
    pub fn is_valid(&self) -> bool {
        self.bid[0].price < self.ask[0].price && self.ask[0].price < self.ask[1].price && self.bid[0].price > self.bid[1].price
    }
}
