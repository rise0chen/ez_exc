use crate::abi::Cex::Pool;
use exc_core::Str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PoolCfg {
    pub ptype: u8,
    pub addr: Str,
    pub base_is_0: bool,
    /// fee: 9970/10000
    pub fee: u16,
}
impl From<&PoolCfg> for Pool {
    fn from(v: &PoolCfg) -> Self {
        Self {
            ptype: v.ptype,
            addr: v.addr.parse().unwrap(),
            fee: v.fee,
        }
    }
}

/// The APIKey definition of DEX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub url: Str,
    pub private_key: Str,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub cex_addr: Str,
    pub quote_addr: Str,
    pub pool_cfg: PoolCfg,
}
