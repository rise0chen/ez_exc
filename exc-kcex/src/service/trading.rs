use super::Kcex;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Kcex {
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
            price.to_f32().unwrap().ln().to_bits(),
            size.to_f32().unwrap().ln().to_bits(),
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
            let (size, is_close) = self.get_order_size(symbol, size, price).await;
            let size = symbol.contract_size(size);
            let price = symbol.contract_price(price, size.is_sign_positive());

            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                external_oid: Some(custom_id),
                side: match (size.is_sign_positive(), is_close) {
                    (true, true) => OrderSide::CloseSell,
                    (true, false) => OrderSide::Buy,
                    (false, true) => OrderSide::CloseBuy,
                    (false, false) => OrderSide::Sell,
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
                order_id: resp.order_id,
                vol: symbol.token_size(resp.vol),
                deal_vol: symbol.token_size(resp.deal_vol),
                deal_avg_price: symbol.token_price(resp.deal_avg_price),
                fee: Fee::Quote(resp.maker_fee + resp.taker_fee),
                state: resp.state,
                side: resp.side,
            }
        };
        Ok(order)
    }
}
