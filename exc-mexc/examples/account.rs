use exc_core::Asset;
use exc_mexc::service::Mexc;
use exc_util::symbol::Symbol;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_mexc=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("MEXC_KEY").unwrap_or_default()).unwrap();
    let mut mexc = Mexc::new(key);

    let balance = mexc.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let symbol = Symbol::spot(Asset::try_from("DOGE").unwrap(), Asset::usdt());
    let balance = mexc.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);

    let symbol = Symbol::derivative(Asset::try_from("DOGE").unwrap(), Asset::usdt());
    let balance = mexc.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
