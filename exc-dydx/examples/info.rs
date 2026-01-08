use exc_dydx::service::Dydx;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_dydx=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("DYDX_KEY").unwrap_or_default()).unwrap();
    let mut dydx = Dydx::new(key).await;

    let mut symbol = Symbol::spot(Asset::try_from("ETH").unwrap(), Asset::usd());
    dydx.perfect_symbol(&mut symbol).await.unwrap();
    let info = dydx.get_funding_rate_history(&symbol, 2).await.unwrap();
    tracing::info!("{:?}", info);
    let info = dydx.get_funding_rate(&symbol).await.unwrap();
    tracing::info!("{:?}", info);
    Ok(())
}
