use core::time::Duration;
use exc_hyperliquid::service::Hyperliquid;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use rust_decimal::Decimal;
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

    let mut symbol = Symbol::derivative(Asset::try_from("xyz:GOLD").unwrap(), Asset::usd());
    symbol.base_id = "110003".into();
    hyperliquid.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(Decimal::new(3, 3), Decimal::new(4880, 0), OrderType::Limit);
    let order_id = hyperliquid.place_order(&symbol, order_req).await.unwrap();
    let order = hyperliquid.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order_id = hyperliquid.cancel_order(order_id).await.unwrap();
    tokio::time::sleep(Duration::from_secs(10)).await;
    let order = hyperliquid.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
