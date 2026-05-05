use core::time::Duration;
use exc_grvt::service::Grvt;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_grvt=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("GRVT_KEY")?)?;
    let mut grvt = Grvt::new(key).await;

    let mut symbol = Symbol::derivative(Asset::try_from("PAXG").unwrap(), Asset::usdt());
    grvt.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(-0.002, 4800.0, OrderType::ImmediateOrCancel);
    let order_id = grvt.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    let order = grvt.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order_id = grvt.cancel_order(order_id).await.unwrap();
    let order = grvt.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
