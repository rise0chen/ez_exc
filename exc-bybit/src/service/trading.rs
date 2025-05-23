use super::Bybit;
use crate::api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Order, OrderId, PlaceOrderRequest};
use tower::ServiceExt;

impl Bybit {
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
            (price as f32).ln().to_bits(),
            (size as f32).ln().to_bits(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64
        );
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.clone()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let req = crate::api::http::trading::PlaceOrderRequest {
            category: symbol.kind,
            symbol: symbol_id,
            order_type: kind.into(),
            side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
            qty: size.abs(),
            market_unit: "baseCoin".into(),
            price,
            order_link_id: Some(custom_id),
        };
        let order_id = self.oneshot(req).await.map(|resp| resp.order_id);
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id.to_string());
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
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let req = crate::api::http::trading::AmendOrderRequest {
            category: symbol.kind,
            symbol: symbol_id,
            order_id,
            order_link_id: custom_order_id,
            qty: order.size,
            price: order.price,
        };
        let resp = self.oneshot(req).await?;
        Ok(OrderId {
            symbol,
            order_id: Some(resp.order_id),
            custom_order_id: resp.order_link_id,
        })
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let req = crate::api::http::trading::CancelOrderRequest {
            category: symbol.kind,
            symbol: symbol_id,
            order_id,
            order_link_id: custom_order_id,
        };
        let resp = self.oneshot(req).await?;
        Ok(OrderId {
            symbol,
            order_id: Some(resp.order_id),
            custom_order_id: resp.order_link_id,
        })
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let req = crate::api::http::trading::GetOrderRequest {
            category: symbol.kind,
            symbol: symbol_id,
            order_id,
            order_link_id: custom_order_id,
        };
        let resp = self.oneshot(req).await?.list.pop();
        resp.map(|resp| Order {
            symbol: resp.symbol,
            order_id: resp.order_id,
            vol: resp.qty,
            deal_vol: resp.cum_exec_qty,
            deal_avg_price: resp.cum_exec_value / resp.cum_exec_qty,
            fee: -resp.cum_exec_fee,
            state: resp.order_status.into(),
            side: resp.side.into(),
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
