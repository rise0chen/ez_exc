use core::time::Duration;
use exc_coinw::service::Coinw;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderId, OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_coinw=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("COINW_KEY")?)?;
    let mut coinw = Coinw::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    coinw.perfect_symbol(&mut symbol).await.unwrap();
    let mut order_req = PlaceOrderRequest::new(-0.001, 5000.0, OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = coinw.place_order(&symbol, order_req).await.unwrap();
    let order = coinw.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(3)).await;
    let order = coinw.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = coinw.cancel_order(order_id).await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    let order = coinw.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
