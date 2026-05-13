use super::Coinw;
use crate::futures_api::types::PositionSide;
use crate::symnol::symbol_id;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, FuturesOpenType, Order, OrderId, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Coinw {
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
            leverage,
            open_type,
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
            todo!();
        } else {
            let price = if kind == OrderType::Market {
                if size.is_sign_positive() {
                    1.01 * price
                } else {
                    0.99 * price
                }
            } else {
                price
            };
            let (size, is_close) = self.get_order_size(symbol, size, price).await;
            let size = symbol.contract_size(size);
            let price = symbol.contract_price(price, size.is_sign_positive());
            if let Some(pos_id) = is_close {
                use crate::futures_api::http::trading::CloseOrderRequest;
                let req = CloseOrderRequest {
                    id: pos_id,
                    third_order_id: Some(custom_id),
                    close_num: size.abs(),
                    order_price: price,
                    position_type: kind.into(),
                    use_almighty_gold: true,
                };
                self.oneshot(req).await.map(|resp| resp.value)
            } else {
                use crate::futures_api::http::trading::PlaceOrderRequest;
                let req = PlaceOrderRequest {
                    instrument: symbol_id,
                    third_order_id: Some(custom_id),
                    direction: if size.is_sign_positive() {
                        PositionSide::Long
                    } else {
                        PositionSide::Short
                    },
                    leverage,
                    position_model: if matches!(open_type, FuturesOpenType::Isolated) { 0 } else { 1 },
                    quantity_unit: 1,
                    quantity: size.abs(),
                    open_price: price,
                    position_type: kind.into(),
                    use_almighty_gold: true,
                };
                self.oneshot(req).await.map(|resp| resp.value)
            }
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id.into_string());
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
            let id = if let Some(id) = &order_id.order_id {
                id.clone()
            } else if order_id.custom_order_id.is_some() {
                self.get_order(order_id.clone()).await?.order_id
            } else {
                return Ok(order_id);
            };
            let req = crate::futures_api::http::trading::CancelOrderRequest { id };
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
            use crate::futures_api::http::trading::{GetOpenOrdersRequest, GetOrderHistoryRequest};
            let req = GetOpenOrdersRequest {
                instrument: symbol_id(&symbol),
                position_type: "plan",
            };
            let resp = self.oneshot(req).await?.rows;
            let resp = resp
                .into_iter()
                .find(|x| x.third_order_id == custom_order_id || Some(x.id.to_string()) == order_id);
            let order = if resp.is_some() {
                resp
            } else {
                let req = GetOrderHistoryRequest {
                    instrument: symbol_id(&symbol),
                    origin_type: "plan",
                };
                let resp = self.oneshot(req).await?.rows;
                resp.into_iter()
                    .find(|x| x.third_order_id == custom_order_id || Some(x.id.to_string()) == order_id)
            };
            let Some(resp) = order else {
                return Err(ExchangeError::OrderNotFound);
            };
            let deal_vol = symbol.token_size(resp.trade_piece.unwrap_or_default());
            let deal_avg_price = symbol.token_price(resp.avg_price.unwrap_or_default());
            let fee = symbol.fee * deal_vol * deal_avg_price;
            Order {
                order_id: resp.id.to_string(),
                vol: symbol.token_size(resp.total_piece),
                deal_vol,
                deal_avg_price,
                fee: Fee::Quote(fee),
                state: resp.order_status.into(),
                side: resp.direction.into(),
            }
        };
        Ok(order)
    }
}
