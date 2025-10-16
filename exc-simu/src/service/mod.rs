mod book;
mod info;
mod trading;

use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct Simu {
    prices: Vec<f64>,
    index: Arc<AtomicUsize>,
}

impl Simu {
    pub fn new(prices: Vec<f64>, interval: u64) -> Self {
        let index = Arc::new(AtomicUsize::new(0));
        let index_set = index.clone();
        tokio::task::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_millis(interval));
            loop {
                ticker.tick().await;
                index_set.fetch_add(1, SeqCst);
            }
        });
        Self { prices, index }
    }
    pub fn price_version(&self) -> u64 {
        self.index.load(SeqCst) as u64
    }
    pub fn price_by_version(&self, v: u64) -> f64 {
        *self.prices.get(v as usize).unwrap()
    }
    pub fn price(&self) -> f64 {
        self.price_by_version(self.price_version())
    }
}
