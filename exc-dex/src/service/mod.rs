mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::abi::Cex::{self, Pool};
use crate::key::Key;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use rand::seq::IndexedRandom;

async fn connect(key: &Key) -> anyhow::Result<DynProvider> {
    let signer: PrivateKeySigner = key.private_key.parse()?;
    let rpc = ProviderBuilder::new().with_simple_nonce_management().wallet(signer);

    let url = if key.url.contains("://") {
        key.url.to_string()
    } else {
        let chain_id = key.url.parse().unwrap();
        let info = crate::three::chain::get_chain(chain_id).await.unwrap();
        info.rpc.choose(&mut rand::rng()).unwrap().url.clone()
    };
    let rpc = if url.starts_with("http") {
        rpc.connect_http(url.parse()?)
    } else if url.starts_with("ws") {
        let ws = WsConnect::new(url.as_str());
        rpc.connect_ws(ws).await?
    } else {
        return Err(anyhow::anyhow!("Unknown rpc url: {}", url));
    };
    Ok(rpc.erased())
}

/// Dex API.
#[derive(Clone)]
pub struct Dex {
    key: Key,
    rpc: DynProvider,

    pub cex: Address,
    pub vault: Address,
    pub quote: Address,
    pub pool: Pool,
}
impl Dex {
    pub async fn new(key: Key) -> Self {
        let cex: Address = key.cex_addr.parse().unwrap();
        let quote: Address = key.quote_addr.parse().unwrap();
        let pool = Pool::from(&key.pool_cfg);

        let rpc = connect(&key).await.unwrap();
        let vault = Cex::new(cex, &rpc)._vault().call().await.unwrap();
        Self {
            key,
            rpc,
            cex,
            vault,
            quote,
            pool,
        }
    }
    pub async fn reconnect(&mut self) -> anyhow::Result<()> {
        let rpc = connect(&self.key).await?;
        self.rpc = rpc;
        Ok(())
    }
}
