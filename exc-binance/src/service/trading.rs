use super::Binance;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, OrderSide, OrderType, PlaceOrderRequest};
use tower::ServiceExt;

impl Binance {
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
                symbol: symbol_id,
                new_client_order_id: Some(custom_id),
                r#type: kind.into(),
                time_in_force: kind.into(),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                quantity: size.abs(),
                price: if kind == OrderType::Market {
                    if size > 0.0 {
                        1.1 * price
                    } else {
                        0.9 * price
                    }
                } else {
                    price
                },
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        } else {
            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                new_client_order_id: Some(custom_id),
                r#type: kind.into(),
                time_in_force: kind.into(),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                quantity: size.abs(),
                price: if kind == OrderType::Market {
                    if size > 0.0 {
                        1.1 * price
                    } else {
                        0.9 * price
                    }
                } else {
                    price
                },
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
                orig_client_order_id: custom_order_id,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.order_id.to_string()),
                custom_order_id: resp.orig_client_order_id,
            }
        } else {
            let req = crate::futures_api::http::trading::CancelOrderRequest {
                order_id,
                orig_client_order_id: custom_order_id,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.order_id.to_string()),
                custom_order_id: resp.client_order_id,
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
                orig_client_order_id: custom_order_id,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            let fee = Fee::Quote(0.001 * resp.cummulative_quote_qty);
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id.to_string(),
                vol: resp.orig_qty.abs(),
                deal_vol: (resp.executed_qty).abs(),
                deal_avg_price: if resp.executed_qty == 0.0 {
                    0.0
                } else {
                    resp.cummulative_quote_qty / resp.executed_qty
                },
                fee,
                state: resp.status,
                side: resp.side,
            }
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                orig_client_order_id: custom_order_id,
                symbol: symbol_id,
            };
            let resp = self.oneshot(req).await?;
            let fee = Fee::Quote(0.001 * resp.cum_quote);
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id.to_string(),
                vol: resp.orig_qty.abs(),
                deal_vol: (resp.executed_qty).abs(),
                deal_avg_price: if resp.executed_qty == 0.0 {
                    0.0
                } else {
                    resp.cum_quote / resp.executed_qty
                },
                fee,
                state: resp.status,
                side: resp.side,
            }
        };
        Ok(order)
    }
}
