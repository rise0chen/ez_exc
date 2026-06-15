use super::Lbank;
use crate::futures_web::types::{OrderSide, OrderStatus, PositionSide};
use crate::symnol::symbol_id;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Lbank {
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
            "{:08x?}{:04x?}{:08x?}",
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u32,
            price.to_i16().unwrap().to_be(),
            price.to_f32().unwrap().ln().to_bits(),
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
            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                exchange_i_d: "Exchange",
                instrument_i_d: symbol_id,
                local_i_d: Some(custom_id),
                direction: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                volume: size.abs(),
                price,
                offset_flag: PositionSide::Open,
                order_price_type: 0,
                order_type: kind.into(),
            };
            self.oneshot(req).await.map(|resp| resp.order_sys_i_d)
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
        if order_id.symbol.is_spot() {
            todo!();
        } else {
            let req = crate::futures_web::http::trading::CancelOrderRequest {
                action_flag: 1,
                order_sys_i_d: order_id.order_id.clone(),
                local_i_d: if order_id.order_id.is_none() {
                    order_id.custom_order_id.clone()
                } else {
                    None
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
            use crate::futures_web::http::trading::{GetCloseOrdersRequest, GetOpenOrdersRequest};
            let order = {
                let req = GetOpenOrdersRequest {
                    exchange_i_d: "Exchange",
                    product_group: "SwapU",
                    instrument_i_d: symbol_id(&symbol),
                    page_size: 20,
                };
                let find_id = |x: &crate::futures_web::http::trading::Order| {
                    Some(&x.order_sys_i_d) == order_id.as_ref() || (custom_order_id.is_some() && custom_order_id == x.local_i_d)
                };
                let resp = self.oneshot(req).await?.data.into_iter().find(find_id);
                if resp.is_some() {
                    resp
                } else {
                    let req = GetCloseOrdersRequest {
                        instrument_i_d: symbol_id(&symbol),
                        page_size: 20,
                    };
                    self.oneshot(req).await?.result_list.into_iter().find(find_id)
                }
            };
            let Some(resp) = order else { return Err(ExchangeError::OrderNotFound) };
            println!("status: {} {:?}", resp.order_status, OrderStatus::from(resp.order_status));

            let deal_vol = symbol.token_size(resp.volume_traded);
            let deal_avg_price = symbol.token_price(if resp.volume_traded == 0.0 {
                0.0
            } else {
                resp.turnover / resp.volume_traded
            });
            let fee = symbol.fee * resp.turnover;
            Order {
                order_id: resp.order_sys_i_d,
                vol: symbol.token_size(resp.volume),
                deal_vol,
                deal_avg_price,
                fee: Fee::Quote(fee),
                state: OrderStatus::from(resp.order_status).into(),
                side: OrderSide::from(resp.direction).into(),
            }
        };
        Ok(order)
    }
}
