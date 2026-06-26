use super::Coinw;
use crate::futures_api::types::{OpenSide, PositionSide};
use crate::symnol::symbol_id;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, FuturesOpenType, Order, OrderId, OrderSide, OrderType, PlaceOrderRequest};
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
        let price = if size > 0.0 && price > symbol.max_price {
            symbol.max_price
        } else if size < 0.0 && price < symbol.min_price {
            symbol.min_price
        } else {
            price
        };
        let custom_id = format!("{}", time::OffsetDateTime::now_utc().unix_timestamp_nanos());
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
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<(), ExchangeError> {
        if order_id.symbol.is_spot() {
            todo!();
        } else {
            let id = if let Some(id) = order_id.order_id {
                id
            } else if order_id.custom_order_id.is_some() {
                self.get_order(order_id).await?.order_id
            } else {
                return Ok(());
            };
            let req = crate::futures_api::http::trading::CancelOrderRequest { id };
            let _resp = self.oneshot(req).await?;
        }
        Ok(())
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
            let filter_order = |x: &crate::futures_api::http::trading::Order| {
                let id = x.order_id.as_ref().unwrap_or(&x.id).to_string();
                Some(id.to_string()) == order_id || (custom_order_id.is_some() && x.third_order_id == custom_order_id)
            };
            let req = GetOpenOrdersRequest {
                instrument: symbol_id(&symbol),
                position_type: "plan",
            };
            let resp = self.oneshot(req).await?.rows;
            let orders: Vec<_> = resp.into_iter().filter(filter_order).collect();
            let mut orders = if !orders.is_empty() {
                orders
            } else {
                let req = GetOrderHistoryRequest {
                    instrument: symbol_id(&symbol),
                    origin_type: "plan",
                    position_model: 1,
                    order_status: "",
                };
                let resp = self.oneshot(req).await?.rows;
                resp.into_iter().filter(filter_order).collect()
            };
            if orders.is_empty() {
                return Err(ExchangeError::OrderNotFound);
            }
            orders.iter_mut().for_each(|x| {
                if x.status.is_close() {
                    x.trade_piece = x.trade_piece.or(x.closed_piece);
                    x.avg_price = x.avg_price.or(x.close_price);
                } else {
                    x.trade_piece = x.trade_piece.or(x.current_piece);
                    x.avg_price = x.avg_price.or(x.open_price);
                }
            });
            let deal_vol: f64 = orders.iter().map(|x| x.trade_piece.unwrap_or(0.0)).sum();
            let deal_value: f64 = orders.iter().map(|x| x.trade_piece.unwrap_or(0.0) * x.avg_price.unwrap_or(0.0)).sum();
            let avg_price = if deal_vol == 0.0 { 0.0 } else { deal_value / deal_vol };
            let deal_vol = symbol.token_size(deal_vol);
            let deal_avg_price = symbol.token_price(avg_price);
            let fee = symbol.fee * deal_vol * deal_avg_price;
            Order {
                order_id: orders[0].order_id.as_ref().unwrap_or(&orders[0].id).to_string(),
                vol: symbol.token_size(orders[0].total_piece),
                deal_vol,
                deal_avg_price,
                fee: Fee::Quote(fee),
                state: orders[0].order_status.into(),
                side: match (orders[0].status, orders[0].direction) {
                    (OpenSide::Open, PositionSide::Long) => OrderSide::Buy,
                    (OpenSide::Open, PositionSide::Short) => OrderSide::Sell,
                    (OpenSide::Close, PositionSide::Long) => OrderSide::CloseBuy,
                    (OpenSide::Close, PositionSide::Short) => OrderSide::CloseSell,
                },
            }
        };
        Ok(order)
    }
}
