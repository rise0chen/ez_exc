use super::Dex;
use crate::abi::{Cex, ERC20};
use crate::error::map_err;
use alloy::primitives::utils::{format_units, parse_units};
use alloy::primitives::Uint;
use alloy::providers::Provider;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, PlaceOrderRequest};

impl Dex {
    pub async fn perfect_symbol(&mut self, symbol: &mut Symbol) -> Result<(), ExchangeError> {
        let base = ERC20::new(symbol.base_id.parse().unwrap(), &self.rpc);
        if symbol.quote_id.is_empty() {
            symbol.quote_id = self.quote.to_string();
        }
        let quote = ERC20::new(symbol.quote_id.parse().unwrap(), &self.rpc);
        let base_decimals = base.decimals().call().await.map_err(map_err)? as i8;
        let quote_decimals = quote.decimals().call().await.map_err(map_err)? as i8;
        let convert_decimals = quote_decimals - base_decimals;
        let multi_price = 10.0f64.powi(convert_decimals as i32);
        if symbol.precision != base_decimals {
            tracing::info!("dex precision from {} to {}", symbol.precision, base_decimals);
            symbol.precision = base_decimals;
        }
        if symbol.multi_price != multi_price {
            tracing::info!("dex multi_price from {} to {}", symbol.multi_price, multi_price);
            symbol.multi_price = multi_price;
        }
        if symbol.multi_size != 1.0 {
            tracing::info!("dex multi_size from {} to {}", symbol.multi_size, 1.0);
            symbol.multi_size = 1.0;
        }
        Ok(())
    }
    pub async fn place_order(&mut self, symbol: &Symbol, data: PlaceOrderRequest) -> Result<OrderId, (OrderId, ExchangeError)> {
        let PlaceOrderRequest {
            size,
            price,
            kind: _,
            leverage: _,
            open_type: _,
        } = data;
        let price = price * symbol.multi_price;
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(String::new()),
        };

        let cex = Cex::new(self.cex, &self.rpc);
        let tx = cex
            .swap(Cex::Route {
                pool: self.pool.clone(),
                zeroForOne: if self.key.pool_cfg.base_is_0 { size < 0.0 } else { size > 0.0 },
                amountSpecified: parse_units(&(-size).to_string(), symbol.precision as u8).unwrap().into(),
                sqrtPriceLimitX96: if self.key.pool_cfg.base_is_0 {
                    Uint::from((price * 2.0f64.powi(32)).sqrt() as u64).checked_shl(80).unwrap()
                } else {
                    Uint::from(((1.0 / price) * 2.0f64.powi(32)).sqrt() as u64).checked_shl(80).unwrap()
                },
            })
            .gas(1_000_000)
            .gas_price(1_000_000_000)
            .send()
            .await;
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
    pub async fn cancel_order(&mut self, _order_id: OrderId) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let OrderId {
            symbol,
            order_id,
            custom_order_id,
        } = order_id;
        let tx_hash = order_id.or(custom_order_id).unwrap_or_default();
        let Ok(tx) = tx_hash.parse() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let Some(tx) = self.rpc.get_transaction_receipt(tx).await.map_err(|e| map_err(e.into()))? else {
            return Err(ExchangeError::OrderNotFound);
        };
        tracing::warn!("{:?}", tx);
        let Some(event) = tx.decoded_log::<Cex::Swap>() else {
            return Err(ExchangeError::OrderNotFound);
        };
        let order = Order {
            symbol: String::new(),
            order_id: tx_hash,
            vol: if self.key.pool_cfg.base_is_0 {
                let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
                size.parse::<f64>().unwrap().abs()
            } else {
                let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
                size.parse::<f64>().unwrap().abs()
            },
            deal_vol: if self.key.pool_cfg.base_is_0 {
                let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
                size.parse::<f64>().unwrap().abs()
            } else {
                let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
                size.parse::<f64>().unwrap().abs()
            },
            deal_avg_price: if self.key.pool_cfg.base_is_0 {
                let quote = format_units(event.data.amount1, symbol.precision as u8).unwrap();
                let size = format_units(event.data.amount0, symbol.precision as u8).unwrap();
                quote.parse::<f64>().unwrap().abs() / size.parse::<f64>().unwrap().abs() / symbol.multi_price
            } else {
                let quote = format_units(event.data.amount0, symbol.precision as u8).unwrap();
                let size = format_units(event.data.amount1, symbol.precision as u8).unwrap();
                quote.parse::<f64>().unwrap().abs() / size.parse::<f64>().unwrap().abs() / symbol.multi_price
            },
            fee: Fee::Quote(0.0),
            state: OrderStatus::Filled,
            side: if self.key.pool_cfg.base_is_0 {
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
            },
        };
        Ok(order)
    }
}
