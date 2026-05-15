use core::time::Duration;
use exc_toobit::service::Toobit;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_toobit=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("TOOBIT_KEY")?)?;
    let mut toobit = Toobit::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("PAXG").unwrap(), Asset::usdt());
    toobit.perfect_symbol(&mut symbol).await.unwrap();
    let mut order_req = PlaceOrderRequest::new(0.001, 4000.0, OrderType::ImmediateOrCancel);
    order_req.set_leverage(20.0);
    let order_id = toobit.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    let order = toobit.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order = toobit.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = toobit.cancel_order(order_id).await.unwrap();
    let order = toobit.get_order(order_id).await;
    tracing::info!("{:?}", order);
    Ok(())
}
