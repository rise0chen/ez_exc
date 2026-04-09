use core::time::Duration;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use exc_weex::service::Weex;
use rust_decimal::Decimal;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_weex=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("WEEX_KEY")?)?;
    let mut weex = Weex::new(key);

    let symbol = Symbol::derivative(Asset::try_from("PAXG").unwrap(), Asset::usdt());
    let mut order_req = PlaceOrderRequest::new(Decimal::new(1, 3), Decimal::new(4000, 0), OrderType::ImmediateOrCancel);
    order_req.set_leverage(20.0);
    let order_id = weex.place_order(&symbol, order_req).await.unwrap_or_else(|e| e.0);
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = weex.cancel_order(order_id).await.unwrap();
    let order = weex.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
