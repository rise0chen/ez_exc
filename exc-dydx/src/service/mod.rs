mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use chrono::{TimeDelta, Utc};
use dydx::config::ClientConfig;
use dydx::indexer::{IndexerClient, IndexerConfig};
use dydx::node::{NodeClient, NodeConfig, Wallet};

/// Dydx API.
#[derive(Clone)]
pub struct Dydx {
    key: Key,
    client_cfg: NodeConfig,
    indexer_cfg: IndexerConfig,
    time_delta: TimeDelta,
}
impl Dydx {
    pub async fn new(key: Key) -> Self {
        exc_util::init();
        let config = ClientConfig::from_file(key.config.as_str()).await.unwrap();
        let indexer = IndexerClient::new(config.indexer.clone());
        let time_delta = indexer.utility().get_time().await.unwrap().iso - Utc::now();
        tracing::info!("dydx time_delta: {time_delta}");
        Self {
            key,
            client_cfg: config.node,
            indexer_cfg: config.indexer,
            time_delta,
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
