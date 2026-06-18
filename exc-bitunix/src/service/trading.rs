use super::Bitunix;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, PlaceOrderRequest};
use tower::ServiceExt;

impl Bitunix {
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
        let custom_id = format!("{}", 116 * time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 100);
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

            if self.key.web_key.is_some() {
                use crate::futures_web::http::trading::PlaceOrderRequest;
                use crate::futures_web::types::*;
                let req = PlaceOrderRequest {
                    symbol: symbol_id,
                    client_id: Some(custom_id),
                    side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                    reduction_only: is_close,
                    effect_type: kind.into(),
                    order_unit: 1,
                    use_percentage: false,
                    amount: size.abs(),
                    front_amount: size.abs(),
                    price,
                    coupon_close: false,
                };
                self.oneshot(req).await.map(|resp| {
                    if resp.client_id.is_some() {
                        ret.custom_order_id = resp.client_id;
                    }
                    resp.order_id
                })
            } else {
                use crate::futures_api::http::trading::PlaceOrderRequest;
                use crate::futures_api::types::*;
                let req = PlaceOrderRequest {
                    symbol: symbol_id,
                    client_id: Some(custom_id),
                    side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                    trade_side: if is_close { TradeSide::Close } else { TradeSide::Open },
                    order_type: kind.into(),
                    effect: kind.into(),
                    qty: size.abs(),
                    price,
                };
                self.oneshot(req).await.map(|resp| resp.order_id)
            }
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
        let order = if symbol.is_spot() {
            todo!();
        } else {
            let client_id = if order_id.is_none() { custom_order_id.clone() } else { None };
            if self.key.web_key.is_some() {
                use crate::futures_web::http::trading::CancelOrderRequest;
                let req = CancelOrderRequest {
                    symbol: symbol_id,
                    order_id: order_id.clone().or(client_id),
                };
                let _ = self.oneshot(req).await?;
            } else {
                use crate::futures_api::http::trading::{CancelOrderRequest, GetOrderRequest};
                let req = CancelOrderRequest {
                    symbol: symbol_id,
                    order_list: vec![GetOrderRequest {
                        order_id: order_id.clone(),
                        client_id,
                    }],
                };
                let _ = self.oneshot(req).await?;
            }
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
            let custom_order_id = if order_id.is_none() { custom_order_id } else { None };
            let req = GetOrderRequest {
                order_id,
                client_id: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                order_id: resp.order_id,
                vol: symbol.token_size(resp.qty),
                deal_vol: symbol.token_size(resp.trade_qty),
                deal_avg_price: symbol.token_price(resp.price),
                fee: Fee::Quote(resp.fee),
                state: resp.status.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
