use exc_lighter::service::Lighter;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_lighter=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("LIGHTER_KEY").unwrap_or_default()).unwrap();
    let mut lighter = Lighter::new(key);
    lighter.run();
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    symbol.base_id = String::from("92");
    let info = lighter.get_funding_rate_history(&symbol, 2).await.unwrap();
    assert!(info[0].time > info[1].time + 58 * 60 * 1000);
    tracing::info!("{:?}", info);
    let rate = lighter.get_funding_rate(&symbol).await.unwrap();
    assert!(rate.time > info[0].time + 58 * 60 * 1000);
    tracing::info!("{:?}", rate);
    let info = lighter.get_index_price(&symbol).await.unwrap();
    tracing::info!("{:?}", info);
    Ok(())
}
