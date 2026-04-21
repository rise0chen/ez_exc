use super::Dex;
use crate::abi::Cex;
use crate::abi::Cex::Place;
use crate::error::map_err;
use alloy::eips::BlockId;
use alloy::primitives::utils::{format_units, parse_units};
use alloy::primitives::Uint;
use alloy::providers::Provider;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, PlaceOrderRequest};
use rust_decimal::prelude::ToPrimitive;

impl Dex {
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind: _,
            leverage: _,
            open_type: _,
        } = data;
        let size = symbol.contract_size(size);
        let price = symbol.contract_price(price, size.is_sign_positive());
        let price = price.to_f64().unwrap() * 10.0f64.powi((symbol.precision_price - symbol.precision) as i32);
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: None,
        };
        let price_limit = if self.key.pool_cfg.base_is_0 {
            Uint::from((price * 2.0f64.powi(128)).sqrt() as u128).saturating_shl(32)
        } else {
            Uint::from(((1.0 / price) * 2.0f64.powi(128)).sqrt() as u128).saturating_shl(32)
        };

        let gas_price = self.key.gas_price as u128;
        let cex = Cex::new(self.cex, &self.rpc);
        let amount = parse_units(&(-size).to_string(), symbol.precision as u8).unwrap().get_signed();
        let pool = self.pool.clone().zero_for_one(if self.key.pool_cfg.base_is_0 {
            size.is_sign_negative()
        } else {
            size.is_sign_positive()
        });
        let place = Place::new(price_limit, amount.try_into().unwrap());
        let mut call = cex
            .swap(pool.into_underlying(), place.into_underlying())
            .gas(self.key.gas_limit)
            .max_fee_per_gas(70 * gas_price)
            .max_priority_fee_per_gas(gas_price);
        match self.rpc.estimate_gas(call.as_ref().clone()).block(BlockId::pending()).await {
            Ok(gas) => {
                if gas > self.key.gas_limit {
                    return Err((ret, ExchangeError::Other(anyhow::anyhow!("gas too much!"))));
                }
                call = call.gas(gas * 13 / 10);
            }
            Err(e) => return Err((ret, map_err(e.into()))),
        }
        let tx = call.send().await;
        let tx = match tx {
            Ok(tx) => tx,
            Err(e) => return Err((ret, map_err(e))),
        };
        let tx = tx.register().await;
        match tx {
            Ok(tx) => {
                let tx_hash = tx.tx_hash().to_string();
                ret.order_id = Some(tx_hash.clone());
                ret.custom_order_id = Some(tx_hash);
                Ok(ret)
            }
            Err(e) => Err((ret, map_err(e.into()))),
        }
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        if order_id.custom_order_id.as_deref().unwrap_or_default() == "" && order_id.order_id.as_deref().unwrap_or_default() == "" {
            Ok(order_id)
        } else {
            Err(ExchangeError::Forbidden(anyhow::anyhow!("")))
        }
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let tx_hash = order_id.or(custom_order_id).unwrap_or_default();
        let mut order = Order {
            order_id: tx_hash,
            vol: 0.0,
            deal_vol: 0.0,
            deal_avg_price: 0.0,
            fee: Fee::Quote(0.0),
            state: OrderStatus::New,
            side: OrderSide::Buy,
        };
        if order.order_id.is_empty() {
            order.state = OrderStatus::Canceled;
            return Ok(order);
        }
        let Ok(tx) = order.order_id.parse() else {
            order.state = OrderStatus::Canceled;
            return Ok(order);
        };
        let Some(tx) = self.rpc.get_transaction_receipt(tx).await.map_err(|e| map_err(e.into()))? else {
            return Ok(order);
        };
        let gas = tx.gas_used as u128 * tx.effective_gas_price + tx.blob_gas_used.unwrap_or(0) as u128 * tx.blob_gas_price.unwrap_or(0);
        order.fee = Fee::Quote(gas as f64 * symbol.fee_coin);
        let Some(event) = tx.decoded_log::<Cex::Swap>() else {
            order.state = OrderStatus::Filled;
            return Ok(order);
        };
        order.vol = if self.key.pool_cfg.base_is_0 {
            let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
            size.parse::<f64>().unwrap().abs()
        } else {
            let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
            size.parse::<f64>().unwrap().abs()
        };
        order.deal_vol = if self.key.pool_cfg.base_is_0 {
            let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
            size.parse::<f64>().unwrap().abs()
        } else {
            let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
            size.parse::<f64>().unwrap().abs()
        };
        order.deal_avg_price = if self.key.pool_cfg.base_is_0 {
            let quote = format_units(event.data.amount1, symbol.precision as u8).unwrap();
            let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
            let p = quote.parse::<f64>().unwrap().abs() / size.parse::<f64>().unwrap().abs();
            p / 10.0f64.powi((symbol.precision_price - symbol.precision) as i32)
        } else {
            let quote = format_units(event.data.amount0, symbol.precision as u8).unwrap();
            let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
            let p = quote.parse::<f64>().unwrap().abs() / size.parse::<f64>().unwrap().abs();
            p / 10.0f64.powi((symbol.precision_price - symbol.precision) as i32)
        };
        order.state = OrderStatus::Filled;
        order.side = if self.key.pool_cfg.base_is_0 {
            if event.data.amount0.is_positive() {
                OrderSide::Sell
            } else {
                OrderSide::Buy
            }
        } else {
            //
            if event.data.amount1.is_positive() {
                OrderSide::Sell
            } else {
                OrderSide::Buy
            }
        };
        Ok(order)
    }
}
