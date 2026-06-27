use super::Pacifica;
use crate::futures_api::types::*;
use crate::symnol::symbol_id;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, PlaceOrderRequest};
use tower::ServiceExt;

impl Pacifica {
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
        let custom_id = uuid::Uuid::new_v4().to_string();
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
                side: if size.is_sign_positive() { OrderSide::Bid } else { OrderSide::Ask },
                reduce_only: false,
                tif: kind.into(),
                amount: size.abs(),
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
            let order_id = order_id.and_then(|x| x.parse().ok());
            let req = CancelOrderRequest {
                symbol: symbol_id(&symbol),
                order_id,
                client_order_id: if order_id.is_none() { custom_order_id } else { None },
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
        use crate::futures_api::http::trading::{GetOpenOrdersRequest, GetOrderHistoryRequest};
        let req = GetOpenOrdersRequest {
            account: self.key.account.to_string(),
        };
        let resp = self
            .oneshot(req)
            .await?
            .into_iter()
            .filter(|x| Some(x.order_id.to_string()) == order_id || (custom_order_id.is_some() && x.client_order_id == custom_order_id))
            .max_by_key(|x| x.created_at);
        let resp = if resp.is_some() {
            resp
        } else {
            let req = GetOrderHistoryRequest {
                account: self.key.account.to_string(),
                limit: 20,
            };
            self.oneshot(req)
                .await?
                .into_iter()
                .filter(|x| Some(x.order_id.to_string()) == order_id || (custom_order_id.is_some() && x.client_order_id == custom_order_id))
                .max_by_key(|x| x.created_at)
        };
        let Some(resp) = resp else {
            return Err(ExchangeError::OrderNotFound);
        };
        let price = resp.average_filled_price.unwrap_or(resp.initial_price);
        Ok(Order {
            order_id: resp.order_id.to_string(),
            vol: symbol.token_size(resp.initial_amount),
            deal_vol: symbol.token_size(resp.filled_amount),
            deal_avg_price: symbol.token_price(price),
            fee: Fee::Quote(symbol.fee * resp.filled_amount * price),
            state: resp.order_status.unwrap_or_default().into(),
            side: resp.side.into(),
        })
    }
}
