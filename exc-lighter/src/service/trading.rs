use super::Lighter;
use crate::futures_api::types::*;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, PlaceOrderRequest};
use exc_util::types::order::{OrderSide, OrderType};
use lighter_rs::types::{CancelOrderTxReq, CreateOrderTxReq, TransactOpts};
use lighter_rs::LighterError;
use rust_decimal::prelude::ToPrimitive;
use tower::ServiceExt;

impl Lighter {
    async fn get_transact_opts(&self) -> TransactOpts {
        TransactOpts {
            from_account_index: Some(self.key.account_index),
            api_key_index: Some(self.key.key_index),
            expired_at: (time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64 + 600_000,
            nonce: self.ws.get_account(&self.key.account_index.to_string()).await.map(|x| x.nonce as i64 + 1),
            dry_run: false,
        }
    }
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
        let custom_id = (time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;
        let ret = OrderId {
            symbol: symbol.clone(),
            order_id: Some(custom_id.to_string()),
            custom_order_id: Some(custom_id.to_string()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let order_id = if symbol.is_spot() {
            todo!();
        } else {
            let mut base_amount = size.abs();
            base_amount.set_scale(0).unwrap();
            let mut price = price;
            price.set_scale(0).unwrap();
            let req = CreateOrderTxReq {
                market_index: symbol_id as u8,
                client_order_index: custom_id,
                base_amount: base_amount.to_i64().unwrap(),
                price: price.to_u32().unwrap(),
                is_ask: size.is_sign_negative() as u8,
                order_type: OrderKind::from(kind) as u8,
                time_in_force: TimeInForce::from(kind) as u8,
                reduce_only: false as u8,
                trigger_price: 0,
                order_expiry: if matches!(kind, OrderType::Unknown | OrderType::Limit | OrderType::LimitMaker) {
                    custom_id + 60 * 60 * 1000
                } else {
                    0
                },
            };
            let tx = match self.tx.create_order(&req, Some(self.get_transact_opts().await)).await {
                Ok(req) => req,
                Err(e) => return Err((ret, ExchangeError::Other(e.into()))),
            };
            match self.tx.send_transaction(&tx).await {
                Ok(res) => Ok(res),
                Err(e) => {
                    let bad_nonce = if let LighterError::ApiError(s) = &e {
                        s.contains("nonce")
                    } else {
                        matches!(e, LighterError::NonceTooLow(_))
                    };
                    if bad_nonce {
                        let tx = match self.tx.create_order(&req, None).await {
                            Ok(req) => req,
                            Err(e) => return Err((ret, ExchangeError::Other(e.into()))),
                        };
                        self.tx.send_transaction(&tx).await
                    } else {
                        Err(e)
                    }
                }
            }
        };
        match order_id {
            Ok(_) => Ok(ret),
            Err(e) => Err((ret, ExchangeError::Other(e.into()))),
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
            let Some(order_id) = order_id else {
                return Err(ExchangeError::OrderNotFound);
            };
            let req = CancelOrderTxReq {
                market_index: symbol_id as u8,
                index: order_id.parse().unwrap(),
            };
            let req = match self.tx.cancel_order(&req, Some(self.get_transact_opts().await)).await {
                Ok(req) => req,
                Err(e) => return Err(ExchangeError::Other(e.into())),
            };
            let resp = self.tx.send_transaction(&req).await;
            if let Err(e) = resp {
                return Err(ExchangeError::Other(e.into()));
            }
            OrderId {
                symbol,
                order_id: Some(order_id),
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
                auth: self.key.read.to_string(),
                account_index: self.key.account_index,
                market_id: symbol_id,
                limit: 10,
                active: true,
            };
            let mut orders = self.oneshot(req).await?.orders;
            orders.retain(|x| {
                (custom_order_id.is_some() && custom_order_id.as_ref() == Some(&x.client_order_id))
                    || (order_id.is_some() && (order_id == Some(x.order_index.to_string()) || order_id.as_ref() == Some(&x.client_order_id)))
            });
            if orders.is_empty() {
                let req = GetOrderRequest {
                    auth: self.key.read.to_string(),
                    account_index: self.key.account_index,
                    market_id: symbol_id,
                    limit: 10,
                    active: false,
                };
                orders = self.oneshot(req).await?.orders;
                orders.retain(|x| {
                    (custom_order_id.is_some() && custom_order_id.as_ref() == Some(&x.client_order_id))
                        || (order_id.is_some() && (order_id == Some(x.order_index.to_string()) || order_id.as_ref() == Some(&x.client_order_id)))
                });
            }
            let order = orders.pop();
            order
                .map(|x| Order {
                    order_id: x.order_index.to_string(),
                    vol: x.initial_base_amount,
                    deal_vol: x.filled_base_amount,
                    deal_avg_price: if x.filled_base_amount == 0.0 {
                        0.0
                    } else {
                        x.filled_quote_amount / x.filled_base_amount
                    },
                    fee: Fee::Quote(0.0),
                    state: x.status.into(),
                    side: if x.is_ask { OrderSide::Sell } else { OrderSide::Buy },
                })
                .ok_or(ExchangeError::OrderNotFound)
        };
        order
    }
}
