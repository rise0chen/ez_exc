use exc_extended::service::Extended;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_extended=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("EXTENDED_KEY").unwrap_or_default()).unwrap();
    let mut extended = Extended::new(key);

    let mut symbol = Symbol::spot(Asset::try_from("ETH").unwrap(), Asset::usd());
    extended.perfect_symbol(&mut symbol).await.unwrap();
    let bid_ask = extended.get_depth(&symbol, 5).await.unwrap();
    assert!(bid_ask.is_valid());
    tracing::info!("{:?}", bid_ask);
    tracing::info!("{:?}", bid_ask.depth_price(500.0));
    Ok(())
}
