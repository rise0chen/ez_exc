use super::Hyperliquid;
use exc_util::error::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::order::{AmendOrder, Fee, Order, OrderId, OrderSide, OrderStatus, OrderType, PlaceOrderRequest};
use hypersdk::hypercore::{OidOrCloid, OrderGrouping, OrderResponseStatus, PrivateKeySigner, Side};
use time::OffsetDateTime;

impl Hyperliquid {
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
        let custom_id = OffsetDateTime::now_utc().unix_timestamp_nanos();
        let mut ret = OrderId {
            symbol: symbol.clone(),
            order_id: None,
            custom_order_id: Some(custom_id.to_string()),
        };
        let signer: PrivateKeySigner = self.key.secret_key.parse().unwrap();
        use hypersdk::hypercore::{BatchOrder, OrderRequest, OrderTypePlacement, TimeInForce};
        let order = BatchOrder {
            orders: vec![OrderRequest {
                asset: symbol.base_id.parse().unwrap(),
                is_buy: size.is_sign_positive(),
                limit_px: price.round_sf(5).unwrap(),
                sz: size.abs(),
                reduce_only: false,
                order_type: OrderTypePlacement::Limit {
                    tif: match kind {
                        OrderType::Unknown | OrderType::Limit => TimeInForce::Gtc,
                        OrderType::Market => TimeInForce::FrontendMarket,
                        OrderType::LimitMaker => TimeInForce::Alo,
                        OrderType::ImmediateOrCancel => TimeInForce::Ioc,
                        OrderType::FillOrKill => TimeInForce::Ioc,
                    },
                },
                cloid: custom_id.to_be_bytes().into(),
            }],
            grouping: OrderGrouping::Na,
        };
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let result = match self.http.place(&signer, order, nonce, None, None).await {
            Ok(mut d) => match d.pop() {
                Some(d) => d,
                None => {
                    return Err((ret, ExchangeError::OrderNotFound));
                }
            },
            Err(e) => {
                return Err((ret, ExchangeError::Other(anyhow::anyhow!("{}", e))));
            }
        };
        let order_id = match result {
            OrderResponseStatus::Success => todo!(),
            OrderResponseStatus::Resting { oid, cloid: _ } => oid,
            OrderResponseStatus::Filled { total_sz: _, avg_px: _, oid } => oid,
            OrderResponseStatus::Error(e) => {
                if e.contains("Order could not immediately match against any resting orders") {
                    ret.custom_order_id = None;
                    return Ok(ret);
                }
                return Err((ret, ExchangeError::Other(anyhow::anyhow!(e))));
            }
        };
        ret.order_id = Some(order_id.to_string());
        Ok(ret)
    }
    fn order_id(&self, order_id: &OrderId) -> Option<OidOrCloid> {
        if let Some(id) = &order_id.custom_order_id {
            if let Ok(id) = id.parse::<i128>() {
                return Some(OidOrCloid::Right(id.to_be_bytes().into()));
            }
        }
        if let Some(id) = &order_id.order_id {
            if let Ok(id) = id.parse() {
                return Some(OidOrCloid::Left(id));
            }
        }
        None
    }
    pub async fn amend_order(&mut self, _order: AmendOrder) -> Result<OrderId, ExchangeError> {
        todo!();
    }
    pub async fn cancel_order(&mut self, order_id: OrderId) -> Result<OrderId, ExchangeError> {
        use hypersdk::hypercore::{BatchCancel, BatchCancelCloid, Cancel, CancelByCloid};
        let asset: u32 = order_id.symbol.base_id.parse().unwrap();
        let signer: PrivateKeySigner = self.key.secret_key.parse().unwrap();
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let Some(oid) = self.order_id(&order_id) else {
            return Ok(order_id);
        };
        let resp = match oid {
            OidOrCloid::Right(cid) => {
                let batch = BatchCancelCloid {
                    cancels: vec![CancelByCloid { asset, cloid: cid }],
                };
                match self.http.cancel_by_cloid(&signer, batch, nonce, None, None).await {
                    Ok(mut d) => d.pop(),
                    Err(e) => {
                        return Err(ExchangeError::Other(anyhow::anyhow!("{}", e.message())));
                    }
                }
            }
            OidOrCloid::Left(id) => {
                let batch = BatchCancel {
                    cancels: vec![Cancel {
                        asset: asset as usize,
                        oid: id,
                    }],
                };
                match self.http.cancel(&signer, batch, nonce, None, None).await {
                    Ok(mut d) => d.pop(),
                    Err(e) => {
                        return Err(ExchangeError::Other(anyhow::anyhow!("{}", e)));
                    }
                }
            }
        };
        match resp {
            Some(d) => {
                if let OrderResponseStatus::Error(e) = d {
                    return Err(ExchangeError::Other(anyhow::anyhow!(e)));
                }
            }
            None => {
                return Err(ExchangeError::OrderNotFound);
            }
        }
        Ok(order_id)
    }
    pub async fn get_order(&mut self, order_id: OrderId) -> Result<Order, ExchangeError> {
        let Some(oid) = self.order_id(&order_id) else {
            return Ok(Order {
                state: OrderStatus::Canceled,
                ..Default::default()
            });
        };
        let resp = self.http.order_status(self.key.user.parse().unwrap(), oid).await?;
        let Some(resp) = resp else {
            return Err(ExchangeError::OrderNotFound);
        };
        if resp.status.is_rejected() {
            return Ok(Order {
                state: OrderStatus::Canceled,
                ..Default::default()
            });
        }
        let mut fills = self.http.user_fills(self.key.user.parse().unwrap()).await?;
        fills.retain(|x| OidOrCloid::Left(x.oid) == oid || (x.cloid.is_some() && x.cloid == oid.right()));
        let (val, size) = fills
            .iter()
            .fold((0.0, 0.0), |(val, size), x| (val + (x.px * x.sz).as_f64(), size + x.sz.as_f64()));
        let fee = fills.iter().map(|x| x.fee.as_f64()).sum();
        let fee = if fills.is_empty() || fills[0].fee_token.contains("USD") {
            Fee::Quote(fee)
        } else {
            Fee::Base(fee)
        };
        let symbol = order_id.symbol;
        Ok(Order {
            order_id: resp.order.oid.to_string(),
            vol: symbol.token_size(resp.order.orig_sz.as_f64()),
            deal_vol: symbol.token_size(size),
            deal_avg_price: if size == 0.0 { 0.0 } else { symbol.token_price(val / size) },
            fee,
            state: if resp.status.is_finished() {
                OrderStatus::Filled
            } else {
                OrderStatus::New
            },
            side: if matches!(resp.order.side, Side::Bid) {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            },
        })
    }
}
