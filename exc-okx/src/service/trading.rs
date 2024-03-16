use super::Okx;
use crate::api::types::OrderSide;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Order, OrderId, PlaceOrderRequest};
use tower::ServiceExt;

impl Okx {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage: _,
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

        let inst_id = crate::symnol::symbol_id(symbol);
        let req = crate::api::http::trading::PlaceOrderRequest {
            inst_id,
            td_mode: open_type.into(),
            side: if size > 0.0 { OrderSide::Buy } else { OrderSide::Sell },
            ord_type: kind.into(),
            sz: size.abs(),
            px: price,
            cl_ord_id: Some(custom_id),
        };
        let order_id = self.oneshot(req).await.map(|mut resp| resp.pop().map(|x| x.ord_id));
        match order_id {
            Ok(id) => {
                ret.order_id = id;
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
        let inst_id = crate::symnol::symbol_id(&symbol);
        let req = crate::api::http::trading::GetOrderRequest {
            inst_id,
            ord_id: order_id,
            cl_ord_id: custom_order_id,
        };
        let resp = self.oneshot(req).await?.pop();
        resp.map(|resp| Order {
            symbol: resp.inst_id,
            order_id: resp.ord_id,
            price: resp.px.parse().unwrap_or(0.0),
            vol: resp.sz,
            deal_vol: resp.acc_fill_sz,
            deal_avg_price: resp.avg_px.parse().unwrap_or(0.0),
            state: resp.state.into(),
            order_type: resp.ord_type.into(),
            side: resp.side.into(),
        })
        .ok_or(ExchangeError::OrderNotFound)
    }
}
