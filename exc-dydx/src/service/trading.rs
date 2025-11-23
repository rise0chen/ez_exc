use super::Dydx;
use bigdecimal::{BigDecimal, ToPrimitive, Zero as _};
use chrono::{TimeDelta, Utc};
use core::time::Duration;
use dydx::indexer::{types::ApiOrderStatus, ClientId, OrderSide as DydxSide, OrderStatus as DydxStatus};
use dydx::node::{OrderBuilder, OrderGoodUntil, OrderId as PlaceId, OrderSide as PlaceSide};
use dydx_proto::dydxprotocol::clob::order::TimeInForce;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, OrderType, PlaceOrderRequest};
use rust_decimal::Decimal;

fn order_side(side: DydxSide) -> OrderSide {
    if side == DydxSide::Buy {
        OrderSide::Buy
    } else {
        OrderSide::Sell
    }
}
fn order_status(status: ApiOrderStatus) -> OrderStatus {
    match status {
        ApiOrderStatus::OrderStatus(order_status) => match order_status {
            DydxStatus::Open => OrderStatus::New,
            DydxStatus::Filled => OrderStatus::Filled,
            DydxStatus::Canceled => OrderStatus::Canceled,
            DydxStatus::BestEffortCanceled => OrderStatus::PartiallyFilled,
            DydxStatus::Untriggered => OrderStatus::New,
        },
        ApiOrderStatus::BestEffort(_) => OrderStatus::New,
    }
}

impl Dydx {
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
        let mut client = self.client().await;
        let custom_id = ClientId::random();
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.0.to_string()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let mut account = self.wallet().account(0, &mut client).await.map_err(|e| (ret.clone(), e.into()))?;
        let subaccount = account.subaccount(0).map_err(|e| (ret.clone(), e.into()))?;
        let market = self
            .indexer()
            .markets()
            .get_perpetual_market(&symbol_id)
            .await
            .map_err(|e| (ret.clone(), e.into()))?;
        let mut order = OrderBuilder::new(market, subaccount);
        let qty = size.abs();
        order = order.limit(
            if size.is_sign_positive() { PlaceSide::Buy } else { PlaceSide::Sell },
            BigDecimal::new(price.mantissa().into(), price.scale() as i64),
            BigDecimal::new(qty.mantissa().into(), qty.scale() as i64),
        );
        match kind {
            OrderType::Unknown => todo!(),
            OrderType::Limit => {
                let until = Utc::now() + TimeDelta::days(1);
                order = order.time_in_force(TimeInForce::Unspecified).until(until).long_term();
            }
            OrderType::Market => {
                let price = if size.is_sign_positive() {
                    (Decimal::new(101, 2) * price).trunc_with_scale(price.scale())
                } else {
                    (Decimal::new(99, 2) * price).trunc_with_scale(price.scale())
                };
                let until = client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20);
                order = order
                    .price(BigDecimal::new(price.mantissa().into(), price.scale() as i64))
                    .time_in_force(TimeInForce::Ioc)
                    .until(until.clone())
                    .short_term();
                while client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))? < until {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            OrderType::LimitMaker => {
                let until = Utc::now() + TimeDelta::days(1);
                order = order.time_in_force(TimeInForce::PostOnly).until(until).long_term();
            }
            OrderType::ImmediateOrCancel => {
                let until = client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20);
                order = order.time_in_force(TimeInForce::Ioc).until(until.clone()).short_term();
                while client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))? < until {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            OrderType::FillOrKill => {
                let until = client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20);
                order = order.time_in_force(TimeInForce::FillOrKill).until(until.clone()).short_term();
                while client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))? < until {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
        let (order_id, order) = order.build(custom_id).map_err(|e| (ret.clone(), e.into()))?;
        ret.order_id = Some(
            [
                order_id.clob_pair_id.to_string(),
                order_id.order_flags.to_string(),
                order_id.client_id.to_string(),
                matches!(kind, OrderType::Limit | OrderType::LimitMaker).to_string(),
            ]
            .join(","),
        );
        let _tx_hash = client
            .place_order(&mut account, order)
            .await
            .map_err(|e| (ret.clone(), ExchangeError::UnexpectedResponseType(e.to_string())))?;
        Ok(ret)
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let mut client = self.client().await;
        let mut account = self.wallet().account(0, &mut client).await?;
        let subaccount = account.subaccount(0)?;

        let mut order_id_split = order_id.order_id.as_ref().unwrap().split(",");
        let place_id = PlaceId {
            subaccount_id: Some(subaccount.into()),
            clob_pair_id: order_id_split.next().unwrap().parse().unwrap(),
            order_flags: order_id_split.next().unwrap().parse().unwrap(),
            client_id: order_id_split.next().unwrap().parse().unwrap(),
        };
        let is_long: bool = order_id_split.next().unwrap().parse().unwrap();
        let until: OrderGoodUntil = if is_long {
            (Utc::now() + TimeDelta::days(1)).into()
        } else {
            client.latest_block_height().await?.ahead(20).into()
        };
        client
            .cancel_order(&mut account, place_id, until)
            .await
            .map_err(|e| ExchangeError::UnexpectedResponseType(e.to_string()))?;
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let order_id = order_id.order_id.unwrap();
        let mut ret = Order {
            symbol: String::new(),
            order_id,
            vol: 0.0,
            deal_vol: 0.0,
            deal_avg_price: 0.0,
            fee: Fee::Quote(0.0),
            state: OrderStatus::New,
            side: OrderSide::Unknown,
        };
        let client_id = ret.order_id.split(",").nth(2).unwrap().parse::<u32>().unwrap();
        let account = self.wallet().account_offline(0)?;
        let subaccount = account.subaccount(0)?;
        let orders = self.indexer().accounts().get_subaccount_orders(&subaccount, None).await?;
        let order = orders.into_iter().find(|x| x.client_id.0 == client_id);
        let Some(order) = order else {
            ret.state = OrderStatus::Canceled;
            return Ok(ret);
        };
        ret.symbol = order.ticker.0;
        ret.vol = order.size.to_f64().unwrap();
        ret.deal_vol = order.total_filled.to_f64().unwrap();
        ret.deal_avg_price = order.price.to_f64().unwrap();
        ret.state = order_status(order.status);
        ret.side = order_side(order.side);
        if !ret.state.is_finished() {
            return Ok(ret);
        }

        let mut vol = BigDecimal::zero();
        let mut value = BigDecimal::zero();
        let mut fee = BigDecimal::zero();
        let fills = self.indexer().accounts().get_subaccount_fills(&subaccount, None).await?;
        for fill in fills {
            if fill.order_id.as_ref() != Some(&order.id) {
                continue;
            }
            vol += &fill.size;
            value += &fill.size * &fill.price.0;
            fee += fill.fee;
        }
        if !vol.is_zero() {
            ret.deal_avg_price = (value / vol).to_f64().unwrap_or(ret.deal_avg_price);
            ret.fee = Fee::Quote(fee.to_f64().unwrap_or(0.0));
        }
        Ok(ret)
    }
}
