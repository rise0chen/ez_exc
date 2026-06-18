use exc_lbank::service::Lbank;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_lbank=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("LBANK_KEY").unwrap_or_default()).unwrap();
    let mut lbank = Lbank::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("PEPE").unwrap(), Asset::usdt());
    lbank.perfect_symbol(&mut symbol).await.unwrap();
    let info = lbank.get_funding_rate_history(&symbol, 2).await.unwrap();
    assert!(info[0].time > info[1].time + 58 * 60 * 1000);
    tracing::info!("{:?}", info);
    let rate = lbank.get_funding_rate(&symbol).await.unwrap();
    assert!(rate.time > info[0].time + 58 * 60 * 1000);
    assert!(rate.time < info[0].time + 25 * 60 * 60 * 1000);
    tracing::info!("{:?}", rate);
    let info = lbank.get_index_price(&symbol).await.unwrap();
    tracing::info!("{:?}", info);
    Ok(())
}
