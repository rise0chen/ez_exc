use exc_kcex::service::Kcex;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_kcex=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("KCEX_KEY").unwrap_or_default()).unwrap();
    let mut kcex = Kcex::new(key);

    let symbol = Symbol::derivative(Asset::try_from("BTC").unwrap(), Asset::usdt());
    let bid_ask = kcex.get_depth(&symbol, 4).await.unwrap();
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
