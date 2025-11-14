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

    let mut symbol = Symbol::derivative(Asset::try_from("ETH").unwrap(), Asset::usd());
    dydx.perfect_symbol(&mut symbol).await.unwrap();
    let bid_ask = dydx.get_depth(&symbol, 4).await.unwrap();
    assert!(bid_ask.is_valid());
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
