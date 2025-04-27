use exc_okx::service::Okx;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_okx=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("OKX_KEY").unwrap_or_default()).unwrap();
    let mut okx = Okx::new(key);

    let symbol = Symbol::spot(Asset::try_from("BTC").unwrap(), Asset::usdt());
    let bid_ask = okx.get_depth(&symbol, 4).await.unwrap();
    tracing::info!("{:?}", bid_ask);

    let symbol = Symbol::derivative(Asset::try_from("BTC").unwrap(), Asset::usdt());
    let bid_ask = okx.get_depth(&symbol, 4).await.unwrap();
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
