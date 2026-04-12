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

    let symbol = Symbol::spot(Asset::try_from("MXSOL").unwrap(), Asset::usdt());
    let rate = lighter.get_st_rate(&symbol).await.unwrap();
    tracing::info!("{:?}", rate);
    Ok(())
}
