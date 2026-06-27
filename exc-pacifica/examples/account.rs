use exc_pacifica::service::Pacifica;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_pacifica=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("PACIFICA_KEY").unwrap_or_default()).unwrap();
    let mut pacifica = Pacifica::new(key);

    let balance = pacifica.get_balance().await.unwrap();
    tracing::info!("{:?}", balance);

    let symbol = Symbol::derivative(Asset::try_from("XAU").unwrap(), Asset::usd());
    let balance = pacifica.get_position(&symbol).await.unwrap();
    tracing::info!("{}: {:?}", symbol, balance);
    Ok(())
}
