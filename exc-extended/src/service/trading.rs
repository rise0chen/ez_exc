use super::Extended;
use crate::futures_api::types::*;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, PlaceOrderRequest};
use rust_crypto_lib_base::starknet_messages::{self, OffChainMessage, StarknetDomain};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::{Decimal, RoundingStrategy};
use starknet_core::types::Felt;
use time::{Duration, OffsetDateTime};
use tower::ServiceExt;

static DOMAIN: StarknetDomain = StarknetDomain {
    name: "Perpetuals",
    version: "v0",
    chain_id: "SN_MAIN",
    revision: 1,
};

impl Extended {
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

        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        let symbol_id = crate::symnol::symbol_id(symbol);
        let mut req = crate::futures_api::http::trading::PlaceOrderRequest {
            market: symbol_id,
            id: Some(custom_id),
            side: if size.is_sign_positive() { OrderSide::Buy } else { OrderSide::Sell },
            qty: size.abs(),
            price,
            fee: symbol.fee,
            reduce_only: false,
            post_only: kind.is_post_only(),
            r#type: kind.into(),
            time_in_force: kind.into(),
            expiry_epoch_millis: now + Duration::days(60).whole_milliseconds() as u64,
            settlement: Default::default(),
            nonce: now as u32,
            self_trade_protection_level: "CLIENT",
            builder_fee: None,
            builder_id: None,
        };
        req.settlement.collateral_position = self.key.vault.to_string();
        req.settlement.stark_key = self.key.public.to_string();
        let starknet_order = starknet_messages::Order {
            position_id: starknet_messages::PositionId { value: self.key.vault },
            base_asset_id: starknet_messages::AssetId {
                value: Felt::from_hex(&symbol.base_id).unwrap(),
            },
            base_amount: {
                let mut s = size;
                s.rescale(symbol.base_precision as u32);
                s.set_scale(0).unwrap();
                s.as_i128() as i64
            },
            quote_asset_id: starknet_messages::AssetId {
                value: Felt::from_hex(&symbol.quote_id).unwrap(),
            },
            quote_amount: {
                let mut s = -size * price;
                s.rescale(symbol.quote_precision as u32);
                s.set_scale(0).unwrap();
                s.as_i128() as i64
            },
            fee_asset_id: starknet_messages::AssetId {
                value: Felt::from_hex(&symbol.quote_id).unwrap(),
            },
            fee_amount: {
                let s = size.abs() * price * Decimal::from_f64(req.fee + req.builder_fee.unwrap_or_default()).unwrap();
                let mut s = s.round_dp_with_strategy(symbol.quote_precision as u32, RoundingStrategy::ToPositiveInfinity);
                s.rescale(symbol.quote_precision as u32);
                s.set_scale(0).unwrap();
                s.as_i128() as u64
            },
            expiration: starknet_messages::Timestamp {
                seconds: (req.expiry_epoch_millis + Duration::days(14).whole_milliseconds() as u64).div_ceil(1000),
            },
            salt: req.nonce.into(),
        };
        let message_hash = starknet_order.message_hash(&DOMAIN, Felt::from_hex(&self.key.public).unwrap()).unwrap();
        let signature = self.key.sign(message_hash).unwrap();
        req.settlement.signature.r = signature.r.to_hex_string();
        req.settlement.signature.s = signature.s.to_hex_string();
        let order_id = self.oneshot(req).await;
        match order_id {
            Ok(id) => {
                ret.order_id = Some(id.id.to_string());
                Ok(ret)
            }
            Err(e) => Err((ret, e)),
        }
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<(), ExchangeError> {
        let OrderId {
            symbol: _,
            order_id,
            custom_order_id,
        } = order_id;
        use crate::futures_api::http::trading::CancelOrderRequest;
        let order_id = order_id.and_then(|x| x.parse().ok());
        let req = CancelOrderRequest {
            external_id: if order_id.is_none() { custom_order_id } else { None },
            order_id,
        };
        let _ = self.oneshot(req).await?;
        Ok(())
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        use crate::futures_api::http::trading::GetOrderRequest;
        let req = GetOrderRequest {
            external_id: custom_order_id,
            order_id,
        };
        let Some(resp) = self.oneshot(req).await?.0.pop() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let price = resp.average_price.or(resp.price).unwrap_or_default();
        Ok(Order {
            order_id: resp.id.to_string(),
            vol: symbol.token_size(resp.qty),
            deal_vol: symbol.token_size(resp.filled_qty),
            deal_avg_price: symbol.token_price(price),
            fee: Fee::Quote(resp.payed_fee.unwrap_or_default()),
            state: resp.status.into(),
            side: resp.side.into(),
        })
    }
}
