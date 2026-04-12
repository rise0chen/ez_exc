mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use crate::ws::Ws;
use core::time::Duration;
use hypersdk::hypercore::{self, HttpClient};
use std::sync::Arc;

/// Hyperliquid API.
#[derive(Clone)]
pub struct Hyperliquid {
    key: Key,
    http: Arc<HttpClient>,
    ws: Ws,
}
impl Hyperliquid {
    pub fn new(key: Key) -> Self {
        let http = Arc::new(hypercore::mainnet());
        let ws = Ws::new(key.market.split(',').map(ToOwned::to_owned).collect());
        Self { key, http, ws }
    }
    pub fn run(&self) {
        if self.ws.symbols[0].is_empty() {
            return;
        }
        let ws = self.ws.clone();
        tokio::spawn(async move {
            loop {
                let ret = ws.run().await;
                ws.clear();
                tracing::info!("hyperliquid ws exit: {ret:?}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }
}
