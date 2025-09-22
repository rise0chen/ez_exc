use super::Kcex;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, PlaceOrderRequest};
use tower::ServiceExt;

impl Kcex {
    pub async fn perfect_symbol(&mut self, _symbol: &mut Symbol) -> Result<(), ExchangeError> {
        Ok(())
    }
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage,
            open_type,
        } = data;
        let custom_id = format!(
            "{:08x?}{:08x?}{:016x?}",
            (price as f32).ln().to_bits(),
            (size as f32).ln().to_bits(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64
        );
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.clone()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let order_id = if symbol.is_spot() {
            todo!();
        } else {
            let (long, short) = self.get_positions(symbol).await.unwrap_or((0.0, 0.0));
            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                external_oid: Some(custom_id),
                side: if size > 0.0 {
                    // 买
                    if size.abs() > short {
                        OrderSide::Buy
                    } else {
                        OrderSide::CloseSell
                    }
                } else {
                    // 卖
                    if size.abs() > long {
                        OrderSide::Sell
                    } else {
                        OrderSide::CloseBuy
                    }
                },
                r#type: kind,
                vol: size.abs(),
                price,
                open_type,
                leverage,
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id);
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_web::http::trading::CancelOrderRequest;
            let id = order_id.clone().unwrap_or(custom_order_id.clone().unwrap_or(String::new()));
            let req = CancelOrderRequest(vec![id]);
            let _ = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id,
                custom_order_id,
            }
        };
        Ok(order)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_web::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: symbol_id,
                order_id,
                external_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id,
                vol: resp.vol,
                deal_vol: resp.deal_vol,
                deal_avg_price: resp.deal_avg_price,
                fee: Fee::Quote(resp.maker_fee + resp.taker_fee),
                state: resp.state,
                side: resp.side,
            }
        };
        Ok(order)
    }
}
