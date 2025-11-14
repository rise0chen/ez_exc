use core::time::Duration;
use exc_dydx::service::Dydx;
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_dydx=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("DYDX_KEY")?)?;
    let mut dydx = Dydx::new(key).await;

    let mut symbol = Symbol::derivative(Asset::try_from("APE").unwrap(), Asset::usd());
    dydx.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(Decimal::new(20, 0), Decimal::new(3, 1), OrderType::Limit);
    let order_id = dydx.place_order(&symbol, order_req).await.unwrap();
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order_id = dydx.cancel_order(order_id).await.unwrap();
    let order = dydx.get_order(order_id.clone()).await.unwrap();
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(10)).await;
    let order = dydx.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
