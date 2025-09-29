use exc_dex::service::Dex;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_dex=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("DEX_KEY").unwrap_or_default()).unwrap();
    let mut dex = Dex::new(key);

    let mut symbol = Symbol::spot(Asset::try_from("ETH").unwrap(), Asset::usdt());
    symbol.base_id = "0x0872C997B2CB959Baf6F422a856AB91d261E5FDb".into();
    dex.perfect_symbol(&mut symbol).await.unwrap();
    let bid_ask = dex.get_depth(&symbol, 4).await.unwrap();
    tracing::info!("{:?}", bid_ask);
    Ok(())
}
