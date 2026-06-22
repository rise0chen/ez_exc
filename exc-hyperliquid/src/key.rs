use exc_util::asset::Str;
use serde::{Deserialize, Serialize};

/// The APIKey definition of HYPERLIQUID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub market: Str,
    pub builder: Option<Str>,
    pub builder_fee: Option<u32>,
    pub user: Str,
    pub secret_key: Str,
}
