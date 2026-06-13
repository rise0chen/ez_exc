use core::time::Duration;
use exc_bitmex::service::Bitmex;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_bitmex=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("BITMEX_KEY").unwrap_or_default()).unwrap();
    let mut bitmex = Bitmex::new(key);
    bitmex.run();
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut symbol = Symbol::derivative(Asset::try_from("PEPE").unwrap(), Asset::usdt());
    bitmex.perfect_symbol(&mut symbol).await.unwrap();
    loop {
        let bid_ask = bitmex.get_depth(&symbol, 5).await.unwrap();
        assert!(bid_ask.is_valid());
        tracing::info!("{:?}", bid_ask);
        tracing::info!("{:?}", bid_ask.depth_price(500.0));

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
