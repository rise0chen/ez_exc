use super::Mexc;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Order, OrderId, OrderSide, PlaceOrderRequest};
use tower::ServiceExt;

impl Mexc {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage,
            open_type,
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
        let order_id = if symbol.is_spot() {
            use crate::spot_api::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                quantity: size.abs(),
                quote_order_qty: size.abs() * price * 1.05,
                price,
                new_client_order_id: Some(custom_id),
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        } else {
            use crate::futures_web::http::trading::PlaceOrderRequest;
            let req = PlaceOrderRequest {
                symbol: symbol_id,
                external_oid: Some(custom_id),
                side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
                r#type: kind,
                vol: size.abs(),
                price,
                open_type,
                leverage,
            };
            self.oneshot(req).await.map(|resp| resp.order_id)
        };
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id);
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let order = if symbol.is_spot() {
            use crate::spot_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: symbol_id,
                order_id,
                orig_client_order_id: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id,
                price: resp.price,
                vol: resp.orig_qty,
                deal_vol: resp.executed_qty,
                deal_avg_price: resp.cummulative_quote_qty / resp.executed_qty,
                state: resp.status,
                order_type: resp.r#type,
                side: resp.side,
            }
        } else {
            use crate::futures_api::http::trading::GetOrderRequest;
            let req = GetOrderRequest {
                symbol: symbol_id,
                order_id,
                external_oid: custom_order_id,
            };
            let resp = self.oneshot(req).await?;
            Order {
                symbol: resp.symbol,
                order_id: resp.order_id,
                price: resp.price,
                vol: resp.vol,
                deal_vol: resp.deal_vol,
                deal_avg_price: resp.deal_avg_price,
                state: resp.state,
                order_type: resp.order_type,
                side: resp.side,
            }
        };
        Ok(order)
    }
}
