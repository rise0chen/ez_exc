use exc_core::Str;
use serde::{Deserialize, Serialize};
/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub account_index: i64,
    pub market_index: u32,
    pub key_index: u8,
    pub key: Str,
    pub read: Str,
}
