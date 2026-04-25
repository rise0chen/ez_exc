use super::Weex;
use crate::futures_api::http::trading::{GetCloseOrdersRequest, GetOpenOrdersRequest};
use crate::futures_api::types::*;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Weex {
    async fn get_order_size(&mut self, symbol: &Symbol, size: f64, price: f64) -> (f64, bool) {
        let (long, short) = self.get_positions(symbol).await.unwrap_or_default();
        let min_once = symbol.min_once(price);
        if size.is_sign_positive() {
            let want_size = size.abs();
            if want_size < short.size {
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
            if want_size < long.size {
                if long.size - want_size <= 1.1 * min_once {
                    (long.size, true)
                } else {
                    (size, true)
                }
            } else {
                if long.size >= min_once {
                    (long.size, true)
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
            leverage: _,
            open_type: _,
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

            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                new_client_order_id: Some(custom_id),
                side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                position_side: match (size.is_sign_positive(), is_close) {
                    (true, true) | (false, false) => PositionSide::Short,
                    (true, false) | (false, true) => PositionSide::Long,
                },
                r#type: kind.into(),
                time_in_force: kind.into(),
                quantity: size.abs(),
                price,
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
            use crate::futures_api::http::trading::CancelOrderRequest;
            let req = CancelOrderRequest {
                order_id: order_id.clone(),
                orig_client_order_id: custom_order_id.clone(),
            };
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
        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let order = if let Some(id) = order_id {
                let req = GetOrderRequest { order_id: id };
                let resp = self.oneshot(req).await?;
                Some(resp)
            } else if let Some(id) = custom_order_id {
                let symbol_id = crate::symnol::symbol_id(&symbol);
                let req = GetOpenOrdersRequest { symbol: symbol_id };
                let resp = self.oneshot(req).await?.into_iter().find(|x| x.client_order_id.as_deref() == Some(&id));
                if resp.is_some() {
                    resp
                } else {
                    let symbol_id = crate::symnol::symbol_id(&symbol);
                    let req = GetCloseOrdersRequest { symbol: symbol_id };
                    self.oneshot(req).await?.into_iter().find(|x| x.client_order_id.as_deref() == Some(&id))
                }
            } else {
                None
            };
            let Some(resp) = order else { return Err(ExchangeError::OrderNotFound) };
            Order {
                order_id: resp.order_id.to_string(),
                vol: symbol.token_size(resp.orig_qty),
                deal_vol: symbol.token_size(resp.executed_qty),
                deal_avg_price: symbol.token_price(resp.avg_price.unwrap_or_default()),
                fee: Fee::Quote(symbol.fee * resp.cum_quote),
                state: resp.status.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
