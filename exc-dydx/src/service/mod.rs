mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use dydx::config::ClientConfig;
use dydx::indexer::{IndexerClient, IndexerConfig};
use dydx::node::{NodeClient, NodeConfig, Wallet};

/// Dydx API.
#[derive(Clone)]
pub struct Dydx {
    key: Key,
    client_cfg: NodeConfig,
    indexer_cfg: IndexerConfig,
}
impl Dydx {
    pub async fn new(key: Key) -> Self {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let config = ClientConfig::from_file(key.config.as_str()).await.unwrap();
        Self {
            key,
            client_cfg: config.node,
            indexer_cfg: config.indexer,
        }
    }
    pub async fn client(&self) -> NodeClient {
        NodeClient::connect(self.client_cfg.clone()).await.unwrap()
    }
    pub fn indexer(&self) -> IndexerClient {
        IndexerClient::new(self.indexer_cfg.clone())
    }
    pub fn wallet(&self) -> Wallet {
        Wallet::from_mnemonic(&self.key.mnemonic).unwrap()
    }
}
