use core::time::Duration;
use exc_hyperliquid::service::Hyperliquid;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_hyperliquid=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("HYPERLIQUID_KEY")?)?;
    let mut hyperliquid = Hyperliquid::new(key);
    hyperliquid.run();
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut symbol = Symbol::spot(Asset::try_from("FLR").unwrap(), Asset::usd());
    symbol.base_id = "10225".into();
    hyperliquid.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(2000.0, 0.007, OrderType::Limit);
    let order_id = hyperliquid.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    let order = hyperliquid.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order = hyperliquid.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    let order_id = hyperliquid.cancel_order(order_id).await.unwrap();
    tokio::time::sleep(Duration::from_secs(10)).await;
    let order = hyperliquid.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
