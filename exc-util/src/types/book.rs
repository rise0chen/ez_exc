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

pub fn depth_price(book: &[Order], depth: f64) -> f64 {
    let mut remain = depth;
    let mut size = 0.0;
    for order in book {
        let val = order.size * order.price;
        if val >= remain {
            size += remain / order.price;
            remain = 0.0;
            break;
        } else {
            size += order.size;
            remain -= val;
        }
    }
    (depth - remain) / size
}

#[derive(Debug, Default, Clone)]
pub struct Depth {
    pub bid: Vec<Order>,
    pub ask: Vec<Order>,
    pub version: u64,
}
impl Depth {
    pub fn is_valid(&self) -> bool {
        if self.bid.len() < 2 || self.ask.len() < 2 {
            return false;
        }
        self.bid[1].price <= self.ask[1].price && self.ask[0].price < self.ask[1].price && self.bid[0].price > self.bid[1].price
    }
    pub fn depth_price(&self, depth: f64) -> (f64, f64) {
        (depth_price(&self.bid, depth), depth_price(&self.ask, depth))
    }
}
