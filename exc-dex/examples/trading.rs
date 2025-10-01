use core::time::Duration;
use exc_dex::service::Dex;
use exc_util::symbol::{Asset, Symbol};
use exc_util::types::order::{OrderType, PlaceOrderRequest};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_dex=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("DEX_KEY")?)?;
    let mut dex = Dex::new(key).await;

    let mut symbol = Symbol::spot(Asset::try_from("ETH").unwrap(), Asset::usdt());
    symbol.base_id = "0x0872C997B2CB959Baf6F422a856AB91d261E5FDb".into();
    dex.perfect_symbol(&mut symbol).await.unwrap();
    let order_req = PlaceOrderRequest::new(2e2, 5000.0, OrderType::Limit);
    let order_id = dex.place_order(&symbol, order_req).await.unwrap();
    tokio::time::sleep(Duration::from_secs(5)).await;
    let order = dex.get_order(order_id).await.unwrap();
    tracing::info!("{:?}", order);
    Ok(())
}
