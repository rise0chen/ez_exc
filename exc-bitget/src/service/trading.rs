use super::Bitget;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, OrderSide, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tower::ServiceExt;

impl Bitget {
    pub async fn perfect_symbol(&mut self, _symbol: &mut Symbol) -> Result<(), ExchangeError> {
        Ok(())
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
            use crate::api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                category: "SPOT",
                symbol: symbol_id,
                client_oid: Some(custom_id),
                order_type: kind.into(),
                time_in_force: kind.into(),
                side: if size.is_sign_positive() {
                    OrderSide::Buy.into()
                } else {
                    OrderSide::Sell.into()
                },
                qty: size.abs(),
                price: if kind == OrderType::Market {
                    if size.is_sign_positive() {
                        (Decimal::new(101, 2) * price).trunc_with_scale(price.scale())
                    } else {
                        (Decimal::new(99, 2) * price).trunc_with_scale(price.scale())
                    }
                } else {
                    price
                },
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        } else {
            use crate::api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                category: "USDT-FUTURES",
                symbol: symbol_id,
                client_oid: Some(custom_id),
                order_type: kind.into(),
                time_in_force: kind.into(),
                side: if size.is_sign_positive() {
                    OrderSide::Buy.into()
                } else {
                    OrderSide::Sell.into()
                },
                qty: size.abs(),
                price: if kind == OrderType::Market {
                    if size.is_sign_positive() {
                        (Decimal::new(101, 2) * price).trunc_with_scale(price.scale())
                    } else {
                        (Decimal::new(99, 2) * price).trunc_with_scale(price.scale())
                    }
                } else {
                    price
                },
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id.to_string());
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let order_id = if symbol.is_spot() {
            let req = crate::api::http::trading::CancelOrderRequest {
                category: "SPOT",
                order_id,
                client_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.order_id.to_string()),
                custom_order_id: resp.client_oid,
            }
        } else {
            let req = crate::api::http::trading::CancelOrderRequest {
                category: "USDT-FUTURES",
                order_id,
                client_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.order_id.to_string()),
                custom_order_id: resp.client_oid,
            }
        };
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;

        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order = if symbol.is_spot() {
            use crate::api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                category: "SPOT",
                order_id,
                client_oid: custom_order_id,
                symbol: symbol_id,
            };
            let mut resp = self.oneshot(req).await?;
            let fee_detail = resp.fee_detail.pop().unwrap_or_default();
            let fee = if fee_detail.fee_coin == symbol.base.as_str() {
                Fee::Base(fee_detail.fee.parse().unwrap_or(0.0))
            } else if fee_detail.fee_coin == "USDT" {
                Fee::Quote(fee_detail.fee.parse().unwrap_or(0.0))
            } else {
                Fee::Quote(0.0006 * resp.cum_exec_value)
            };
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id.to_string(),
                vol: resp.qty.abs(),
                deal_vol: (resp.cum_exec_qty).abs(),
                deal_avg_price: if resp.cum_exec_qty == 0.0 {
                    0.0
                } else {
                    resp.cum_exec_value / resp.cum_exec_qty
                },
                fee,
                state: resp.order_status.into(),
                side: resp.side.into(),
            }
        } else {
            use crate::api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                category: "USDT-FUTURES",
                order_id,
                client_oid: custom_order_id,
                symbol: symbol_id,
            };
            let mut resp = self.oneshot(req).await?;
            let fee_detail = resp.fee_detail.pop().unwrap_or_default();
            let fee = if fee_detail.fee_coin == symbol.base.as_str() {
                Fee::Base(fee_detail.fee.parse().unwrap_or(0.0))
            } else if fee_detail.fee_coin == "USDT" {
                Fee::Quote(fee_detail.fee.parse().unwrap_or(0.0))
            } else {
                Fee::Quote(0.0006 * resp.cum_exec_value)
            };
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id.to_string(),
                vol: resp.qty.abs(),
                deal_vol: (resp.cum_exec_qty).abs(),
                deal_avg_price: if resp.cum_exec_qty == 0.0 {
                    0.0
                } else {
                    resp.cum_exec_value / resp.cum_exec_qty
                },
                fee,
                state: resp.order_status.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
