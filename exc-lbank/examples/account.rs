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

    let balance = lbank.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let mut symbol = Symbol::derivative(Asset::try_from("GOLD").unwrap(), Asset::usdt());
    lbank.perfect_symbol(&mut symbol).await.unwrap();
    let balance = lbank.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
