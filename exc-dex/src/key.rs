use crate::abi::Cex::Pool;
use exc_core::Str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PoolCfg {
    pub addr: Str,
    pub ptype: u8,
    pub base_is_0: bool,
    /// fee: 9970/10000
    pub fee: u16,
}
impl From<&PoolCfg> for Pool {
    fn from(v: &PoolCfg) -> Self {
        Self {
            addr: v.addr.parse().unwrap(),
            ptype: v.ptype,
            fee: v.fee,
        }
    }
}

/// The APIKey definition of DEX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub url: Str,
    pub private_key: Str,
    pub cex_addr: Str,
    pub quote_addr: Str,
    pub pool_cfg: PoolCfg,
}

impl Key {
    /// Create a new [`Key`].
    pub fn new(url: &str, private_key: &str, cex: &str, quote: &str, pool_cfg: PoolCfg) -> Self {
        Self {
            url: Str::new(url),
            private_key: Str::new(private_key),
            cex_addr: Str::new(cex),
            quote_addr: Str::new(quote),
            pool_cfg,
        }
    }
}
