use crate::futures_api::types::OrderSide;

use super::Aden;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderStatus, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tower::ServiceExt;

impl Aden {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage: _,
            open_type: _,
        } = data;
        let size = symbol.contract_size(size);
        let price = symbol.contract_price(price, size.is_sign_positive());
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
            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                contract: symbol_id,
                text: Some(custom_id),
                size,
                price: if kind == OrderType::Market { Decimal::ZERO } else { price },
                tif: kind.into(),
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

        let order_id = if symbol.is_spot() {
            todo!();
        } else {
            let req = crate::futures_api::http::trading::AmendOrderRequest {
                order_id,
                external_oid: custom_order_id,
                size: symbol.contract_size(order.size),
                price: order.price.map(|x| symbol.contract_price(x, order.size > 0.0)),
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
        let order_id = if symbol.is_spot() {
            todo!();
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

        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                external_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            let deal_vol = (resp.size - resp.left).abs();
            let fee = 0.0005 * deal_vol * symbol.multi_size * resp.fill_price;
            Order {
                order_id: resp.id.to_string(),
                vol: symbol.token_size(resp.size.abs()),
                deal_vol: symbol.token_size(deal_vol),
                deal_avg_price: symbol.token_price(resp.fill_price),
                fee: Fee::Quote(fee),
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
