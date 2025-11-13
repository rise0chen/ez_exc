use core::time::Duration;
use exc_bitunix::service::Bitunix;
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_bitunix=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("BITUNIX_KEY")?)?;
    let mut bitunix = Bitunix::new(key);

    let symbol = Symbol::derivative(Asset::try_from("APE").unwrap(), Asset::usdt());
    let mut order_req = PlaceOrderRequest::new(Decimal::new(20, 0), Decimal::new(3, 1), OrderType::Limit);
    order_req.set_leverage(20.0);
    let order_id = bitunix.place_order(&symbol, order_req).await.unwrap();
    tokio::time::sleep(Duration::from_secs(32)).await;
    let order_id = bitunix.cancel_order(order_id).await.unwrap();
    let order = bitunix.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
