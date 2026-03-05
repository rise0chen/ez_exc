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

    let symbol = Symbol::spot(Asset::try_from("MXSOL").unwrap(), Asset::usdt());
    let rate = aster.get_st_rate(&symbol).await.unwrap();
    tracing::info!("{:?}", rate);
    Ok(())
}
