use exc_core::Asset;
use exc_lighter::service::Lighter;
use exc_util::symbol::Symbol;
use std::env::var;

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

    let balance = lighter.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let mut symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usdt());
    symbol.base_id = String::from("92");
    let balance = lighter.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
