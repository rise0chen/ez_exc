use core::time::Duration;
use exc_aster::service::Aster;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_aster=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("ASTER_KEY")?)?;
    let mut aster = Aster::new(key);

    let symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    let mut order_req = PlaceOrderRequest::new(0.001, 5000.0, OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = aster.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    let order = aster.get_order(order_id.clone()).await.unwrap();
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = aster.cancel_order(order_id).await.unwrap();
    let order = aster.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
