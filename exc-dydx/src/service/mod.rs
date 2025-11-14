mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use dydx::config::ClientConfig;
use dydx::indexer::IndexerClient;
use dydx::node::{NodeClient, Wallet};

/// Dydx API.
pub struct Dydx {
    client: NodeClient,
    indexer: IndexerClient,
    wallet: Wallet,
}
impl Dydx {
    pub async fn new(key: Key) -> Self {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let config = ClientConfig::from_file(key.config.as_str()).await.unwrap();
        let client = NodeClient::connect(config.node).await.unwrap();
        let indexer = IndexerClient::new(config.indexer);
        let wallet = Wallet::from_mnemonic(&key.mnemonic).unwrap();
        Self { client, indexer, wallet }
    }
}
