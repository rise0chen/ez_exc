use exc_bitunix::service::Bitunix;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;
use std::time::Duration;

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
    tokio::time::sleep(Duration::from_secs(2)).await;

    let balance = bitunix.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let mut symbol = Symbol::derivative(Asset::try_from("DOGE").unwrap(), Asset::usdt());
    bitunix.perfect_symbol(&mut symbol).await.unwrap();
    let balance = bitunix.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
