use core::time::Duration;
use exc_lbank::service::Lbank;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_lbank=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("LBANK_KEY")?)?;
    let mut lbank = Lbank::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("PEPE").unwrap(), Asset::usdt());
    lbank.perfect_symbol(&mut symbol).await.unwrap();
    let mut order_req = PlaceOrderRequest::new(10000.0, 2e-6, OrderType::ImmediateOrCancel);
    order_req.set_leverage(20.0);
    let order_id = lbank.place_order(&symbol, order_req).await.unwrap();
    let order = lbank.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(3)).await;
    let order = lbank.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = lbank.cancel_order(order_id).await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    let order = lbank.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
