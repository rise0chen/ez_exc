use core::time::Duration;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use exc_xt::service::Xt;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_xt=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("XT_KEY")?)?;
    let mut xt = Xt::new(key);

    let symbol = Symbol::derivative(Asset::try_from("APE").unwrap(), Asset::usdt());
    let mut order_req = PlaceOrderRequest::new(-20.0, 0.3, OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = xt.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = xt.cancel_order(order_id).await.unwrap();
    let order = xt.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
