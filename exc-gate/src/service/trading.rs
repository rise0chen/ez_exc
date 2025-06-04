use super::Gate;
use crate::spot_api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Order, OrderId, OrderStatus, OrderType, PlaceOrderRequest};
use tower::ServiceExt;

impl Gate {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage,
            open_type: _,
        } = data;
        let custom_id = format!(
            "t-{:08x?}{:04x?}{:016x?}",
            (price as f32).ln().to_bits(),
            (size as i16).to_be(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64
        );
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.clone()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let order_id = if symbol.is_spot() {
            use crate::spot_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                currency_pair: symbol_id,
                text: Some(custom_id),
                r#type: kind.into(),
                time_in_force: kind.into(),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                amount: size.abs(),
                price: if kind == OrderType::Market {
                    if size > 0.0 {
                        1.1 * price
                    } else {
                        0.9 * price
                    }
                } else {
                    price
                },
                auto_borrow: leverage != 1.0,
                auto_repay: leverage != 1.0,
            };
            self.oneshot(req).await.map(|resp| resp.id)
        } else {
            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                contract: symbol_id,
                text: Some(custom_id),
                size: size.round() as i64,
                price: if kind == OrderType::Market { 0.0 } else { price },
                tif: String::from(if kind == OrderType::Market { "ioc" } else { "" }),
            };
            self.oneshot(req).await.map(|resp| resp.id.to_string())
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id);
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn amend_order(&mut self, order: AmendOrder) -> Result<OrderId, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order.id;

        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order_id = if symbol.is_spot() {
            let req = crate::spot_api::http::trading::AmendOrderRequest {
                order_id,
                text: custom_order_id,
                currency_pair: symbol_id,
                amount: order.size,
                price: order.price,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.id.to_string()),
                custom_order_id: resp.text,
            }
        } else {
            let req = crate::futures_api::http::trading::AmendOrderRequest {
                order_id,
                external_oid: custom_order_id,
                size: order.size.map(|x| x as i64),
                price: order.price,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.id.to_string()),
                custom_order_id: resp.text,
            }
        };
        Ok(order_id)
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order_id = if symbol.is_spot() {
            let req = crate::spot_api::http::trading::CancelOrderRequest {
                order_id,
                text: custom_order_id,
                currency_pair: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.id.to_string()),
                custom_order_id: resp.text,
            }
        } else {
            let req = crate::futures_api::http::trading::CancelOrderRequest {
                order_id,
                external_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.id.to_string()),
                custom_order_id: resp.text,
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
            use crate::spot_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                text: custom_order_id,
                currency_pair: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.currency_pair,
                order_id: resp.id.to_string(),
                vol: resp.amount.abs(),
                deal_vol: (resp.filled_amount).abs(),
                deal_avg_price: resp.filled_total / resp.filled_amount,
                fee: resp.gt_taker_fee.unwrap_or(0.0) * resp.filled_total,
                state: resp.status.into(),
                side: resp.side.into(),
            }
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                external_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.contract,
                order_id: resp.id.to_string(),
                vol: resp.size.abs(),
                deal_vol: (resp.size - resp.left).abs(),
                deal_avg_price: resp.fill_price,
                fee: resp.tkfr.unwrap_or(0.0) * (resp.size - resp.left).abs() * resp.fill_price,
                state: match resp.finish_as.as_deref() {
                    None => OrderStatus::New,
                    Some("filled") | Some("ioc") => OrderStatus::Filled,
                    Some("cancelled") => OrderStatus::Canceled,
                    Some(_) => OrderStatus::Unknown,
                },
                side: if resp.size > 0.0 {
                    OrderSide::Buy.into()
                } else {
                    OrderSide::Sell.into()
                },
            }
        };
        Ok(order)
    }
}
