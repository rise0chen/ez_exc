pub use crate::error::ExchangeError;
pub use crate::symbol::Symbol;
pub use crate::types::account::{Balance, Position};
pub use crate::types::book::Depth;
pub use crate::types::earn::StRate;
pub use crate::types::info::FundingRate;
pub use crate::types::order::{Order, OrderId, PlaceOrderRequest};

#[allow(async_fn_in_trait)]
pub trait ExchangeTrait {
    async fn get_balance(&mut self) -> Result<Balance, ExchangeError>;
    async fn get_positions(&mut self, symbol: &Symbol) -> Result<(Position, Position), ExchangeError>;
    async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError>;
    async fn get_index_price(&mut self, symbol: &Symbol) -> Result<f64, ExchangeError>;
    async fn get_funding_rate(&mut self, symbol: &Symbol) -> Result<FundingRate, ExchangeError>;
    async fn get_funding_rate_history(&mut self, symbol: &Symbol, day: u8) -> Result<Vec<FundingRate>, ExchangeError>;
    async fn get_st_rate(&mut self, symbol: &Symbol) -> Result<StRate, ExchangeError>;
    async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError>;
    async fn get_order(&mut self, id: OrderId) -> Result<Order, ExchangeError>;
    async fn place_order(&mut self, symbol: &Symbol, order_req: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)>;
    async fn cancel_order(&mut self, id: OrderId) -> Result<(), ExchangeError>;

    async fn get_position(&mut self, symbol: &Symbol) -> Result<Position, ExchangeError> {
        self.get_positions(symbol).await.map(|(long, short)| {
            let size = long.size - short.size;
            let price = if (long.size + short.size) == 0.0 {
                0.0
            } else {
                (long.size * long.price + short.size * short.price) / (long.size + short.size)
            };
            Position {
                id: String::new(),
                size,
                price,
            }
        })
    }
    async fn get_order_size(&mut self, symbol: &Symbol, size: f64, price: f64) -> (f64, bool) {
        let (long, short) = self.get_positions(symbol).await.unwrap_or_default();
        let min_once = symbol.min_once(price);
        if size.is_sign_positive() {
            let want_size = size.abs();
            if want_size <= short.size {
                if short.size - want_size <= 1.1 * min_once {
                    (short.size, true)
                } else {
                    (size, true)
                }
            } else {
                if short.size >= min_once {
                    (short.size, true)
                } else {
                    (size, false)
                }
            }
        } else {
            let want_size = size.abs();
            if want_size <= long.size {
                if long.size - want_size <= 1.1 * min_once {
                    (-long.size, true)
                } else {
                    (size, true)
                }
            } else {
                if long.size >= min_once {
                    (-long.size, true)
                } else {
                    (size, false)
                }
            }
        }
    }
}
