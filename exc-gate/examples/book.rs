use exc_gate::service::Gate;
use exc_util::symbol::{Asset, Symbol};
use std::env::var;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,exc_gate=trace".into()),
        ))
        .init();

    let key = serde_json::from_str(&var("GATE_KEY").unwrap_or_default()).unwrap();
    let mut gate = Gate::new(key);
    gate.run();
    tokio::time::sleep(Duration::from_secs(2)).await;

    loop {
        let symbol = Symbol::spot(Asset::try_from("BTC").unwrap(), Asset::usdt());
        let bid_ask = gate.get_depth(&symbol, 4).await.unwrap();
        assert!(bid_ask.is_valid());
        tracing::info!("{:?}", bid_ask);

        let symbol = Symbol::derivative(Asset::try_from("BTC").unwrap(), Asset::usdt());
        let bid_ask = gate.get_depth(&symbol, 4).await.unwrap();
        assert!(bid_ask.is_valid());
        tracing::info!("{:?}", bid_ask);

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
