use exc_hyperliquid::service::Hyperliquid;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_hyperliquid=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("HYPERLIQUID_KEY").unwrap_or_default()).unwrap();
    let mut hyperliquid = Hyperliquid::new(key);
    hyperliquid.run();

    let mut symbol = Symbol::derivative(Asset::try_from("xyz:GOLD").unwrap(), Asset::usd());
    symbol.base_id = "110003".into();
    hyperliquid.perfect_symbol(&mut symbol).await.unwrap();
    let bid_ask = hyperliquid.get_depth(&symbol, 4).await.unwrap();
    assert!(bid_ask.is_valid());
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
