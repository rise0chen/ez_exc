use super::Dex;
use crate::abi::Cex;
use crate::error::map_err;
use alloy::eips::BlockId;
use alloy::primitives::utils::format_units;
use alloy::primitives::U160;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::{Depth, Order};

fn price(sqrt_price_x96: U160) -> f64 {
    (sqrt_price_x96.arithmetic_shr(32).to::<u128>() as f64 / 2.0f64.powi(64)).powi(2)
}
fn map_order0(x: &Cex::Order, symbol: &Symbol) -> Order {
    let price = price(x.price);
    let size = format_units(x.amount0, symbol.precision as u8).unwrap();
    Order::new(price / symbol.multi_price, size.parse().unwrap())
}
fn map_order1(x: &Cex::Order, symbol: &Symbol) -> Order {
    let price = 1.0 / price(x.price);
    let size = format_units(x.amount1, symbol.precision as u8).unwrap();
    Order::new(price / symbol.multi_price, size.parse().unwrap())
}

impl Dex {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let cex = Cex::new(self.cex, &self.rpc);
        let depth = cex.getDepth(self.pool.clone(), limit);
        let depth = depth.block(BlockId::pending()).call().await.map_err(map_err)?;
        let (bid, ask) = if self.key.pool_cfg.base_is_0 {
            let bid = depth.bids.iter().map(|x| map_order0(x, symbol)).collect();
            let ask = depth.asks.iter().map(|x| map_order0(x, symbol)).collect();
            (bid, ask)
        } else {
            let bid = depth.asks.iter().map(|x| map_order1(x, symbol)).collect();
            let ask = depth.bids.iter().map(|x| map_order1(x, symbol)).collect();
            (bid, ask)
        };
        let bid_ask = Depth {
            bid,
            ask,
            price: if self.key.pool_cfg.base_is_0 {
                price(depth.price)
            } else {
                1.0 / price(depth.price)
            } / symbol.multi_price,
            version: depth.timestamp.to::<u64>(),
        };
        Ok(bid_ask)
    }
}
