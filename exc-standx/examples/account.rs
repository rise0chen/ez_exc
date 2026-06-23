use exc_standx::service::Standx;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_standx=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("STANDX_KEY").unwrap_or_default()).unwrap();
    let mut standx = Standx::new(key);

    let balance = standx.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usd());
    let balance = standx.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
