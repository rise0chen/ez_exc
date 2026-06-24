use super::Dydx;
use bigdecimal::{BigDecimal, ToPrimitive, Zero as _};
use chrono::{TimeDelta, Utc};
use dydx::indexer::{types::ApiOrderStatus, ClientId, OrderSide as DydxSide, OrderStatus as DydxStatus};
use dydx::node::{OrderBuilder, OrderGoodUntil, OrderId as PlaceId, OrderSide as PlaceSide};
use dydx_proto::dydxprotocol::clob::order::TimeInForce;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{Fee, Order, OrderId, OrderSide, OrderStatus, OrderType, PlaceOrderRequest};

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
        let price = if kind == OrderType::Market {
            if size.is_sign_positive() {
                1.01 * price
            } else {
                0.99 * price
            }
        } else {
            price
        };
        let size = symbol.contract_size(size);
        let price = symbol.contract_price(price, size.is_sign_positive());
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
            OrderType::Unknown | OrderType::Limit => {
                let until = Utc::now() + self.time_delta + TimeDelta::days(1);
                order = order.time_in_force(TimeInForce::Unspecified).until(until).long_term();
            }
            OrderType::Market => {
                let until = Utc::now() + self.time_delta + TimeDelta::seconds(5);
                order = order.price(BigDecimal::new(price.mantissa().into(), price.scale() as i64));
                order = order.time_in_force(TimeInForce::Unspecified).until(until).long_term();
            }
            OrderType::LimitMaker => {
                let until = Utc::now() + self.time_delta + TimeDelta::days(1);
                order = order.time_in_force(TimeInForce::PostOnly).until(until).long_term();
            }
            OrderType::ImmediateOrCancel | OrderType::FillOrKill => {
                let until = Utc::now() + self.time_delta + TimeDelta::seconds(5);
                order = order.time_in_force(TimeInForce::Unspecified).until(until).long_term();
            }
        };
        let (order_id, order) = order.build(custom_id).map_err(|e| (ret.clone(), e.into()))?;
        ret.order_id = Some(
            [
                order_id.clob_pair_id.to_string(),
                order_id.order_flags.to_string(),
                order_id.client_id.to_string(),
            ]
            .join(","),
        );
        let _tx_hash = client
            .place_order(&mut account, order)
            .await
            .map_err(|e| (ret.clone(), ExchangeError::UnexpectedResponseType(e.to_string())))?;
        Ok(ret)
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<(), ExchangeError> {
        if order_id.order_id.as_deref().unwrap_or_default().is_empty() {
            return Err(ExchangeError::OrderNotFound);
        }
        let mut client = self.client().await;
        let mut account = self.wallet().account(0, &mut client).await?;
        let subaccount = account.subaccount(0)?;

        let mut order_id_split = order_id.order_id.as_ref().unwrap().split(",");
        let (pair, flag, id) = match (order_id_split.next(), order_id_split.next(), order_id_split.next()) {
            (Some(pair), Some(flag), Some(id)) => (pair, flag, id),
            _ => return Err(ExchangeError::OrderNotFound),
        };
        let place_id = PlaceId {
            subaccount_id: Some(subaccount.into()),
            clob_pair_id: pair.parse().unwrap(),
            order_flags: flag.parse().unwrap(),
            client_id: id.parse().unwrap(),
        };
        let until: OrderGoodUntil = (Utc::now() + TimeDelta::days(1)).into();
        client
            .cancel_order(&mut account, place_id, until)
            .await
            .map_err(|e| ExchangeError::UnexpectedResponseType(e.to_string()))?;
        Ok(())
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let order_id = order_id.order_id.unwrap_or_default();
        let mut ret = Order {
            order_id,
            vol: 0.0,
            deal_vol: 0.0,
            deal_avg_price: 0.0,
            fee: Fee::Quote(0.0),
            state: OrderStatus::New,
            side: OrderSide::Buy,
        };
        if ret.order_id.is_empty() {
            return Err(ExchangeError::OrderNotFound);
        }
        let mut order_id_split = ret.order_id.split(",");
        let (_pair, _flag, id) = match (order_id_split.next(), order_id_split.next(), order_id_split.next()) {
            (Some(pair), Some(flag), Some(id)) => (pair, flag, id),
            _ => return Err(ExchangeError::OrderNotFound),
        };
        let client_id = id.parse::<u32>().unwrap();
        let account = self.wallet().account_offline(0)?;
        let subaccount = account.subaccount(0)?;
        let orders = self.indexer().accounts().get_subaccount_orders(&subaccount, None).await?;
        let order = orders.into_iter().find(|x| x.client_id.0 == client_id);
        let Some(order) = order else {
            return Ok(ret);
        };
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
