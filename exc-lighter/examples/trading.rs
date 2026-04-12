use core::time::Duration;
use exc_lighter::service::Lighter;
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_lighter=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("LIGHTER_KEY")?)?;
    let mut lighter = Lighter::new(key);
    lighter.run();
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    symbol.base_id = String::from("92");
    let mut order_req = PlaceOrderRequest::new(Decimal::new(-30, 4), Decimal::new(530000, 2), OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = lighter.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    let order = lighter.get_order(order_id.clone()).await;
    tracing::info!("{:?}", order);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = lighter.cancel_order(order_id).await.unwrap();
    let order = lighter.get_order(order_id).await;
    tracing::info!("{:?}", order);
    Ok(())
}
