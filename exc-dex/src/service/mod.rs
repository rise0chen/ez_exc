mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::abi::Cex::Pool;
use crate::key::Key;
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;

/// Dex API.
#[derive(Clone)]
pub struct Dex {
    key: Key,
    rpc: DynProvider,

    pub cex: Address,
    pub quote: Address,
    pub pool: Pool,
}

impl Dex {
    pub fn new(key: Key) -> Self {
        let cex = key.cex_addr.parse().unwrap();
        let quote = key.quote_addr.parse().unwrap();
        let pool = Pool::from(&key.pool_cfg);

        let signer: PrivateKeySigner = key.private_key.parse().unwrap();
        let url = key.url.parse().unwrap();

        let rpc = ProviderBuilder::new().with_cached_nonce_management().wallet(signer).connect_http(url);
        Self {
            key,
            rpc: rpc.erased(),
            cex,
            quote,
            pool,
        }
    }
}
