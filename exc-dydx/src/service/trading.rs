use super::Dydx;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{TimeDelta, Utc};
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
        let custom_id = ClientId::random();
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.0.to_string()),
        };

        let symbol_id = crate::symnol::symbol_id(symbol);
        let mut account = self.wallet.account(0, &mut self.client).await.map_err(|e| (ret.clone(), e.into()))?;
        let subaccount = account.subaccount(0).map_err(|e| (ret.clone(), e.into()))?;
        let market = self
            .indexer
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
                order = order
                    .time_in_force(TimeInForce::Unspecified)
                    .until(Utc::now() + TimeDelta::days(1))
                    .long_term();
            }
            OrderType::Market => {
                let price = if size.is_sign_positive() {
                    (Decimal::new(101, 2) * price).trunc_with_scale(price.scale())
                } else {
                    (Decimal::new(99, 2) * price).trunc_with_scale(price.scale())
                };
                order = order
                    .price(BigDecimal::new(price.mantissa().into(), price.scale() as i64))
                    .time_in_force(TimeInForce::Ioc)
                    .until(self.client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20))
                    .short_term();
            }
            OrderType::LimitMaker => {
                order = order
                    .time_in_force(TimeInForce::PostOnly)
                    .until(Utc::now() + TimeDelta::days(1))
                    .long_term();
            }
            OrderType::ImmediateOrCancel => {
                order = order
                    .time_in_force(TimeInForce::Ioc)
                    .until(self.client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20))
                    .short_term();
            }
            OrderType::FillOrKill => {
                order = order
                    .time_in_force(TimeInForce::FillOrKill)
                    .until(self.client.latest_block_height().await.map_err(|e| (ret.clone(), e.into()))?.ahead(20))
                    .long_term();
            }
        }
        let (order_id, order) = order.build(custom_id).map_err(|e| (ret.clone(), e.into()))?;
        let _tx_hash = self
            .client
            .place_order(&mut account, order)
            .await
            .map_err(|e| (ret.clone(), ExchangeError::UnexpectedResponseType(e.to_string())))?;
        ret.order_id = Some(
            [
                order_id.clob_pair_id.to_string(),
                order_id.order_flags.to_string(),
                order_id.client_id.to_string(),
                matches!(kind, OrderType::Limit | OrderType::LimitMaker).to_string(),
            ]
            .join(","),
        );
        Ok(ret)
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        let mut account = self.wallet.account(0, &mut self.client).await?;
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
            self.client.latest_block_height().await?.ahead(20).into()
        };
        self.client
            .cancel_order(&mut account, place_id, until)
            .await
            .map_err(|e| ExchangeError::UnexpectedResponseType(e.to_string()))?;
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let order_id = order_id.custom_order_id.unwrap();
        let account = self.wallet.account(0, &mut self.client).await?;
        let subaccount = account.subaccount(0)?;
        let orders = self.indexer.accounts().get_subaccount_orders(&subaccount, None).await?;
        let order = orders.into_iter().find(|x| x.client_id.0 == order_id.parse::<u32>().unwrap());
        let Some(order) = order else {
            return Err(ExchangeError::OrderNotFound);
        };
        let mut ret = Order {
            symbol: order.ticker.0,
            order_id,
            vol: order.size.to_f64().unwrap(),
            deal_vol: order.total_filled.to_f64().unwrap(),
            deal_avg_price: order.price.to_f64().unwrap(),
            fee: Fee::Quote(0.0),
            state: order_status(order.status),
            side: order_side(order.side),
        };
        if !ret.state.is_finished() {
            return Ok(ret);
        }
        let fills = self.indexer.accounts().get_subaccount_fills(&subaccount, None).await?;
        let fill = fills.into_iter().find(|x| x.order_id.as_ref() == Some(&order.id));
        if let Some(fill) = fill {
            ret.deal_avg_price = fill.price.to_f64().unwrap_or(ret.deal_avg_price);
            ret.fee = Fee::Quote(fill.fee.to_f64().unwrap_or(0.0))
        }
        Ok(ret)
    }
}
