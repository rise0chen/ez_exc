use super::Grvt;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, OrderType, PlaceOrderRequest};
use grvt_rust_sdk::signer::*;
use grvt_rust_sdk::types::*;

impl Grvt {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind,
            leverage: _,
            open_type: _,
        } = data;
        let size = symbol.contract_size(size);
        let price = symbol.contract_price(price, size.is_sign_positive());
        let custom_id = format!("{}", time::OffsetDateTime::now_utc().unix_timestamp_nanos() as u64 | 2_u64.pow(63));
        let ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.clone()),
        };
        let symbol_id = crate::symnol::symbol_id(symbol);

        let params = SignOrderParams {
            sub_account_id: self.key.account_id.parse().unwrap(),
            is_market: matches!(kind, OrderType::Market),
            time_in_force: match kind {
                OrderType::Unknown | OrderType::Limit => TimeInForce::GoodTillTime,
                OrderType::Market | OrderType::ImmediateOrCancel => TimeInForce::ImmediateOrCancel,
                OrderType::LimitMaker => TimeInForce::AllOrNone,
                OrderType::FillOrKill => TimeInForce::FillOrKill,
            },
            post_only: matches!(kind, OrderType::LimitMaker),
            reduce_only: false,
            legs: vec![SignOrderLeg {
                asset_id: parse_instrument_hash(&symbol.base_id).unwrap(),
                contract_size: {
                    let mut s = size.abs();
                    s.rescale(symbol.quote_id.parse().unwrap());
                    s.set_scale(0).unwrap();
                    s.as_i128() as u64
                },
                limit_price: {
                    let mut p = price;
                    p.rescale(9);
                    p.set_scale(0).unwrap();
                    p.as_i128() as u64
                },
                is_buying_contract: size.is_sign_positive(),
            }],
            nonce: random_nonce(),
            expiration_ns: default_expiration_ns(),
            chain_id: self.http.env.default_chain_id(),
        };
        let priv_key = decode_private_key(&self.key.secret_key).unwrap();
        let signed = sign_order(&params, &priv_key).unwrap();

        let req = CreateOrderRequest {
            order: OrderPayload {
                sub_account_id: self.key.account_id.to_string(),
                is_market: params.is_market,
                time_in_force: params.time_in_force.as_api_str().into(),
                post_only: params.post_only,
                reduce_only: params.reduce_only,
                legs: vec![grvt_rust_sdk::types::OrderLeg {
                    instrument: symbol_id,
                    size: size.abs().to_string(),
                    limit_price: Some(price.to_string()),
                    is_buying_asset: size.is_sign_positive(),
                }],
                signature: signed.signature,
                metadata: Some(OrderMetadata {
                    client_order_id: ret.custom_order_id.clone(),
                    create_time: None,
                    trigger: None,
                    broker: None,
                    is_position_transfer: None,
                    allow_crossing: None,
                }),
                builder: None,
                builder_fee: None,
            },
        };
        let _result = match self.http.create_order_full(&req).await {
            Ok(d) => d,
            Err(e) => {
                return Err((ret, ExchangeError::Other(e.into())));
            }
        };
        Ok(ret)
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let req = CancelOrderRequest {
            sub_account_id: self.key.account_id.to_string(),
            order_id: order_id.order_id.clone(),
            client_order_id: order_id.custom_order_id.clone(),
        };
        let _ = self.http.cancel_order_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?;
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        if order_id.is_none() && custom_order_id.is_none() {
            return Ok(Order {
                state: OrderStatus::Canceled,
                ..Default::default()
            });
        }
        let req = GetOrderRequest {
            sub_account_id: self.key.account_id.to_string(),
            order_id,
            client_order_id: custom_order_id,
        };
        let resp = self.http.get_order_full(&req).await.map_err(|e| ExchangeError::Other(e.into()))?.result;
        let Some(state) = resp.state else {
            return Err(ExchangeError::OrderNotFound);
        };
        let deal_vol = state.traded_size[0];
        let avg_price = state.avg_fill_price[0];
        Ok(Order {
            order_id: resp.order_id.unwrap_or_default(),
            vol: resp.legs[0].size,
            deal_vol,
            deal_avg_price: avg_price,
            fee: Fee::Quote(symbol.fee * deal_vol * avg_price),
            state: match &*state.status {
                "PENDING" | "OPEN" => OrderStatus::New,
                "FILLED" => OrderStatus::Filled,
                "REJECTED" | "CANCELLED" => OrderStatus::Canceled,
                _ => OrderStatus::New,
            },
            side: if resp.legs[0].is_buying_asset { OrderSide::Buy } else { OrderSide::Sell },
        })
    }
}
