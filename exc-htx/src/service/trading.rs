use super::Htx;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, OrderSide, OrderType, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tower::ServiceExt;

impl Htx {
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
            use crate::spot_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                account_id: self.key.account_id,
                source: "spot-api",
                symbol: symbol_id,
                client_order_id: Some(custom_id),
                r#type: (if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell }, kind).into(),
                amount: size.abs(),
                price: if kind == OrderType::Market {
                    if size.is_sign_positive() {
                        (Decimal::new(101, 2) * price).trunc_with_scale(price.scale())
                    } else {
                        (Decimal::new(99, 2) * price).trunc_with_scale(price.scale())
                    }
                } else {
                    price
                },
            };
            self.oneshot(req).await.map(|resp| resp.data)
        } else {
            todo!()
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
        let order_id = if symbol.is_spot() {
            let req = crate::spot_api::http::trading::CancelOrderRequest {
                order_id,
                client_order_id: custom_order_id.clone(),
            };
            let resp = self.oneshot(req).await?;
            OrderId {
                symbol,
                order_id: Some(resp.data),
                custom_order_id,
            }
        } else {
            todo!()
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
            use crate::spot_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                order_id,
                client_order_id: custom_order_id,
            };
            let resp = self.oneshot(req).await?.data;
            Order {
                symbol: resp.symbol,
                order_id: resp.id.to_string(),
                vol: resp.amount.abs(),
                deal_vol: (resp.field_amount).abs(),
                deal_avg_price: if resp.field_amount == 0.0 {
                    0.0
                } else {
                    resp.field_cash_amount / resp.field_amount
                },
                fee: Fee::Quote(resp.field_fees),
                state: resp.state.into(),
                side: resp.r#type.into(),
            }
        } else {
            todo!()
        };
        Ok(order)
    }
}
