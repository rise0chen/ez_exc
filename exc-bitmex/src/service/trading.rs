use super::Bitmex;
use crate::futures_api::types::{OrderSide, PositionSide};
use crate::symnol::symbol_id;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use std::collections::BTreeMap;
use tower::ServiceExt;

impl Bitmex {
    #[allow(unused)]
    async fn get_order_size(&mut self, symbol: &Symbol, size: f64, price: f64) -> (f64, Option<String>) {
        let (long, short) = self.get_positions(symbol).await.unwrap_or_default();
        let min_once = symbol.min_once(price);
        if size.is_sign_positive() {
            let want_size = size.abs();
            if want_size <= short.size {
                if short.size - want_size <= 1.1 * min_once {
                    (short.size, Some(short.id))
                } else {
                    (size, Some(short.id))
                }
            } else {
                if short.size >= min_once {
                    (short.size, Some(short.id))
                } else {
                    (size, None)
                }
            }
        } else {
            let want_size = size.abs();
            if want_size <= long.size {
                if long.size - want_size <= 1.1 * min_once {
                    (-long.size, Some(long.id))
                } else {
                    (size, Some(long.id))
                }
            } else {
                if long.size >= min_once {
                    (-long.size, Some(long.id))
                } else {
                    (size, None)
                }
            }
        }
    }

    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage: _,
            open_type: _,
        } = data;
        let price = if size > 0.0 && price > symbol.max_price {
            symbol.max_price
        } else if size < 0.0 && price < symbol.min_price {
            symbol.min_price
        } else {
            price
        };
        let custom_id = format!(
            "t-{:08x?}{:04x?}{:016x?}",
            price.to_f32().unwrap().ln().to_bits(),
            price.to_i16().unwrap().to_be(),
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
            let price = if kind == OrderType::Market {
                if size.is_sign_positive() { 1.01 * price } else { 0.99 * price }
            } else {
                price
            };
            let size = symbol.contract_size(size);
            let price = symbol.contract_price(price, size.is_sign_positive());
            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                cl_ord_i_d: Some(custom_id),
                side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                strategy: PositionSide::OneWay,
                display_qty: size.abs(),
                order_qty: size.abs(),
                price,
                time_in_force: kind.into(),
            };
            self.oneshot(req).await.map(|resp| resp.order_i_d)
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id);
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        if order_id.symbol.is_spot() {
            todo!();
        } else {
            let req = crate::futures_api::http::trading::CancelOrderRequest {
                order_i_d: if let Some(id) = &order_id.order_id {
                    vec![id.clone()]
                } else {
                    Vec::new()
                },
                cl_ord_i_d: if order_id.order_id.is_none()
                    && let Some(id) = &order_id.custom_order_id
                {
                    vec![id.clone()]
                } else {
                    Vec::new()
                },
            };
            let _resp = self.oneshot(req).await?;
        }
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;

        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let mut filter = BTreeMap::new();
            if let Some(id) = order_id {
                filter.insert("orderID".into(), id);
            }
            if let Some(id) = custom_order_id {
                filter.insert("clOrdID".into(), id);
            }
            let req = GetOrderRequest {
                symbol: symbol_id(&symbol),
                filter,
            };
            let resp = self.oneshot(req).await?.pop();
            let Some(resp) = resp else {
                return Err(ExchangeError::OrderNotFound);
            };
            let deal_vol = symbol.token_size(resp.cum_qty.unwrap_or_default() as f64);
            let deal_avg_price = symbol.token_price(resp.avg_px.unwrap_or_default());
            let fee = symbol.fee * deal_vol * deal_avg_price;
            Order {
                order_id: resp.order_i_d,
                vol: symbol.token_size(resp.order_qty as f64),
                deal_vol,
                deal_avg_price,
                fee: Fee::Quote(fee),
                state: resp.ord_status.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
