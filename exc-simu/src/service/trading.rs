use super::Simu;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{self, AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus};

impl Simu {
    pub async fn place_order(&mut self, symbol: &Symbol, data: order::PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let mut id = OrderId::new(symbol.clone());
        let v = self.price_version();
        id.order_id = Some(format!("{},{}", v, data.size));
        Ok(id)
    }
    pub async fn amend_order(&mut self, order: AmendOrder) -> Result<OrderId, ExchangeError> {
        Ok(order.id)
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let symbol = order_id.symbol.base_id;
        let order_id = order_id.order_id.unwrap();
        let mut id = order_id.split(',');
        let v: u64 = id.next().unwrap().parse().unwrap();
        let size: f64 = id.next().unwrap().parse().unwrap();
        let price = self.price_by_version(v);
        Ok(Order {
            symbol,
            order_id,
            vol: size.abs(),
            deal_vol: size.abs(),
            deal_avg_price: price,
            fee: Fee::Quote(0.0),
            state: OrderStatus::Filled,
            side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
        })
    }
}
