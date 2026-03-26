use exc_bitunix::service::Bitunix;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_bitunix=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("BITUNIX_KEY").unwrap_or_default()).unwrap();
    let mut bitunix = Bitunix::new(key);
    bitunix.run();

    let symbol = Symbol::derivative(Asset::try_from("BTC").unwrap(), Asset::usdt());
    let info = bitunix.get_funding_rate_history(&symbol, 2).await.unwrap();
    assert!(info[0].time > info[1].time + 58 * 60 * 1000);
    tracing::info!("{:?}", info);
    let rate = bitunix.get_funding_rate(&symbol).await.unwrap();
    assert!(rate.time > info[0].time + 58 * 60 * 1000);
    tracing::info!("{:?}", rate);
    assert_eq!(info[0].interval, rate.interval);
    let info = bitunix.get_index_price(&symbol).await.unwrap();
    tracing::info!("{:?}", info);
    Ok(())
}
