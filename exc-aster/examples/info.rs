use exc_aster::service::Aster;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_aster=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("ASTER_KEY").unwrap_or_default()).unwrap();
    let mut aster = Aster::new(key);

    let mut symbol = Symbol::derivative(Asset::try_from("BTC").unwrap(), Asset::usdt());
    aster.perfect_symbol(&mut symbol).await.unwrap();
    let info = aster.get_funding_rate_history(&symbol, 2).await.unwrap();
    assert!(info[0].time > info[1].time + 58 * 60 * 1000);
    tracing::info!("{:?}", info);
    let rate = aster.get_funding_rate(&symbol).await.unwrap();
    assert!(rate.time > info[0].time + 58 * 60 * 1000);
    tracing::info!("{:?}", rate);
    let info = aster.get_index_price(&symbol).await.unwrap();
    tracing::info!("{:?}", info);
    Ok(())
}
