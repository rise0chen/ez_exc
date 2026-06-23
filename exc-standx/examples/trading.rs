use core::time::Duration;
use exc_standx::service::Standx;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_standx=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("STANDX_KEY")?)?;
    let mut standx = Standx::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usd());
    standx.perfect_symbol(&mut symbol).await.unwrap();
    let mut order_req = PlaceOrderRequest::new(-0.001, 5000.0, OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = standx.place_order(&symbol, order_req).await.unwrap();
    let order = standx.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order = standx.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    tracing::info!("{:?}", standx.cancel_order(order_id.clone()).await);
    let order = standx.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order = standx.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    Ok(())
}
