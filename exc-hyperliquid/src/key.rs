use exc_core::Str;
use serde::{Deserialize, Serialize};

/// The APIKey definition of HYPERLIQUID.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub market: Str,
    pub market_index: usize,
    pub user: Str,
    pub secret_key: Str,
}
