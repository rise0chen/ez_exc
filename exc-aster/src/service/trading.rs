use super::Aster;
use crate::futures_api::types::*;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::OrderSide;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Aster {
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
            use crate::futures_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                new_client_order_id: Some(custom_id),
                side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                position_side: PositionSide::Both,
                r#type: kind.into(),
                time_in_force: kind.into(),
                quantity: size.abs(),
                price,
            };
            self.oneshot(req).await
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id.order_id.to_string());
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
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::CancelOrderRequest;
            let req = CancelOrderRequest {
                symbol: symbol_id,
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
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order = if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: symbol_id,
                order_id,
                orig_client_order_id: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id.to_string(),
                vol: resp.orig_qty,
                deal_vol: resp.executed_qty,
                deal_avg_price: resp.avg_price.unwrap_or_default(),
                fee: Fee::Quote(0.0004 * resp.avg_price.unwrap_or_default() * resp.executed_qty),
                state: resp.status.into(),
                side: resp.side,
            }
        };
        Ok(order)
    }
}
