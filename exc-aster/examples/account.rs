use exc_aster::service::Aster;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_aster=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("ASTER_KEY").unwrap_or_default()).unwrap();
    let mut aster = Aster::new(key);

    let balance = aster.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    let balance = aster.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
