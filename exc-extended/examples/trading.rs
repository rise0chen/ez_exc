use core::time::Duration;
use exc_extended::service::Extended;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_extended=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("EXTENDED_KEY")?)?;
    let mut extended = Extended::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("ETH").unwrap(), Asset::usd());
    extended.perfect_symbol(&mut symbol).await.unwrap();
    let mut order_req = PlaceOrderRequest::new(-0.011, 1600.1, OrderType::LimitMaker);
    order_req.set_leverage(20.0);
    let order_id = extended.place_order(&symbol, order_req).await.unwrap();
    let order = extended.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(30)).await;
    let order = extended.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tracing::info!("{:?}", extended.cancel_order(order_id.clone()).await);
    let order = extended.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(20)).await;
    let order = extended.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    Ok(())
}
