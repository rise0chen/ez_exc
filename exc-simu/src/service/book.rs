use super::Simu;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};

impl Simu {
    pub async fn get_depth(&mut self, _symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let version = self.price_version();
        let price = self.price();
        let size = 1e32;
        let order = Order { price, size };
        let orders = vec![order; limit as usize];
        let depth = Depth {
            bid: orders.clone(),
            ask: orders,
            version,
        };
        Ok(depth)
    }
}
