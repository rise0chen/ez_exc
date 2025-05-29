use core::time::Duration;
use exc_mexc::service::Mexc;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_mexc=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("MEXC_KEY")?)?;
    let mut mexc = Mexc::new(key);

    let mut symbol = Symbol::spot(Asset::try_from("APE").unwrap(), Asset::usdt());
    mexc.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(20.0, 0.3, OrderType::Limit);
    let order_id = mexc.place_order(&symbol, order_req).await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    let order_id = mexc.cancel_order(order_id).await.unwrap();
    let order = mexc.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);

    let symbol = Symbol::derivative(Asset::try_from("APE").unwrap(), Asset::usdt());
    let mut order_req = PlaceOrderRequest::new(20.0, 0.3, OrderType::Limit);
    order_req.set_leverage(10.0);
    let order_id = mexc.place_order(&symbol, order_req).await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    let order_id = mexc.cancel_order(order_id).await.unwrap();
    let order = mexc.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
