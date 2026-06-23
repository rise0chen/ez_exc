use super::Standx;
use crate::futures_api::types::*;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Standx {
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
        let size = symbol.contract_size(size);
        let price = symbol.contract_price(price, size.is_sign_positive());
        let custom_id = format!(
            "{:08x?}{:08x?}{:016x?}",
            price.to_f32().unwrap().ln().to_bits(),
            size.to_f32().unwrap().ln().to_bits(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64
        );
        let ret = OrderId {
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
                cl_ord_id: Some(custom_id),
                side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                reduce_only: false,
                order_type: kind.into(),
                time_in_force: kind.into(),
                qty: size.abs(),
                price,
            };
            self.oneshot(req).await
        };
        match order_id {
            Ok(_id) => Ok(ret),
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<(), ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        if symbol.is_spot() {
            todo!();
        } else {
            use crate::futures_api::http::trading::CancelOrderRequest;
            let req = CancelOrderRequest {
                order_id,
                cl_ord_id: custom_order_id,
            };
            let _ = self.oneshot(req).await?;
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
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                cl_ord_id: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                order_id: resp.id.to_string(),
                vol: symbol.token_size(resp.qty),
                deal_vol: symbol.token_size(resp.fill_qty),
                deal_avg_price: symbol.token_price(resp.fill_avg_price.unwrap_or_default()),
                fee: Fee::Quote(resp.fee),
                state: resp.status.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
