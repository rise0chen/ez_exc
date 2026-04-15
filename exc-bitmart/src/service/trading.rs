use super::Bitmart;
use crate::futures_api::types::*;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tower::ServiceExt;

impl Bitmart {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage,
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
                client_order_id: Some(custom_id),
                side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind.into(),
                leverage: Decimal::from_f64_retain(leverage).unwrap(),
                mode: kind.into(),
                size: size.abs().floor().to_u64().unwrap(),
                price,
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
                client_order_id: custom_order_id.clone(),
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
            use crate::futures_api::http::trading::{GetCloseOrdersRequest, GetOpenOrdersRequest, GetTradesRequest};
            let order = if let Some(id) = &order_id {
                let req = GetOpenOrdersRequest { symbol: symbol_id.clone() };
                let resp = self.oneshot(req).await?.into_iter().find(|x| &x.order_id == id);
                if resp.is_some() {
                    resp
                } else {
                    let req = GetCloseOrdersRequest {
                        symbol: symbol_id.clone(),
                        client_order_id: custom_order_id.clone(),
                    };
                    self.oneshot(req).await?.into_iter().find(|x| &x.order_id == id)
                }
            } else if let Some(id) = &custom_order_id {
                let req = GetOpenOrdersRequest { symbol: symbol_id.clone() };
                let resp = self.oneshot(req).await?.into_iter().find(|x| x.client_order_id.as_deref() == Some(id));
                if resp.is_some() {
                    resp
                } else {
                    let req = GetCloseOrdersRequest {
                        symbol: symbol_id.clone(),
                        client_order_id: custom_order_id.clone(),
                    };
                    self.oneshot(req).await?.into_iter().find(|x| x.client_order_id.as_deref() == Some(id))
                }
            } else {
                None
            };
            let Some(resp) = order else { return Err(ExchangeError::OrderNotFound) };

            let req = GetTradesRequest {
                symbol: symbol_id,
                order_id: order_id.clone(),
                client_order_id: custom_order_id,
            };
            let fee = self.oneshot(req).await?.iter().map(|x| x.paid_fees).sum();

            Order {
                order_id: resp.order_id,
                vol: resp.size,
                deal_vol: resp.deal_size,
                deal_avg_price: resp.deal_avg_price,
                fee: Fee::Quote(fee),
                state: resp.state.into(),
                side: resp.side.into(),
            }
        };
        Ok(order)
    }
}
