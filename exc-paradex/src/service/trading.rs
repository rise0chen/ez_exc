use super::Paradex;
use bigdecimal::ToPrimitive as _;
use core::time::Duration;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, OrderType, PlaceOrderRequest};
use paradex::structs::{OrderInstruction, OrderUpdate, Side};

impl Paradex {
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
        let order_request = paradex::structs::OrderRequest {
            instruction: match kind {
                OrderType::Unknown | OrderType::Limit => OrderInstruction::GTC,
                OrderType::Market => OrderInstruction::IOC,
                OrderType::LimitMaker => OrderInstruction::POST_ONLY,
                OrderType::ImmediateOrCancel => OrderInstruction::IOC,
                OrderType::FillOrKill => OrderInstruction::IOC,
            },
            market: symbol_id,
            price: Some(price),
            side: if size.is_sign_positive() { Side::BUY } else { Side::SELL },
            size: size.abs(),
            order_type: paradex::structs::OrderType::LIMIT,
            client_id: ret.custom_order_id.clone(),
            flags: vec![],
            recv_window: None,
            stp: None,
            trigger_price: None,
        };
        let result = self
            .http
            .create_order(order_request)
            .await
            .map_err(|e| (ret.clone(), ExchangeError::Other(e.into())))?;
        ret.order_id = Some(result.id);
        Ok(ret)
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        match (&order_id.order_id, &order_id.custom_order_id) {
            (_, Some(custom_order_id)) => {
                self.http
                    .cancel_order_by_client_id(custom_order_id.clone())
                    .await
                    .map_err(|e| ExchangeError::Other(e.into()))?;
            }
            (Some(order_id), None) => {
                self.http
                    .cancel_order(order_id.clone())
                    .await
                    .map_err(|e| ExchangeError::Other(e.into()))?;
            }
            (None, None) => return Err(ExchangeError::OrderNotFound),
        };
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let symbol_id = crate::symnol::symbol_id(&symbol);
        let start = chrono::Utc::now() - Duration::from_hours(24);
        let mut req = vec![("market".into(), symbol_id)];
        if let Some(id) = custom_order_id {
            req.push(("client_id".into(), id));
        }
        let mut resp = self
            .http
            .request_cursor::<OrderUpdate>("/v1/orders-history".into(), Some(req), Some(start), None, true)
            .await
            .map_err(|e| ExchangeError::Other(e.into()))?;
        if let Some(id) = order_id {
            resp.retain(|x| x.id == id);
        }
        let Some(resp) = resp.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let deal_vol = (resp.size - resp.remaining_size).as_f64();
        let avg_price = if resp.avg_fill_price.is_nan() { 0.0 } else { resp.avg_fill_price };
        Ok(Order {
            order_id: resp.id,
            vol: resp.size.as_f64(),
            deal_vol,
            deal_avg_price: avg_price,
            fee: Fee::Quote(symbol.fee * deal_vol * avg_price),
            state: if matches!(resp.status, paradex::structs::OrderStatus::CLOSED) {
                OrderStatus::Filled
            } else {
                OrderStatus::New
            },
            side: if matches!(resp.side, Side::BUY) {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            },
        })
    }
}
