use exc_core::Str;
use serde::{Deserialize, Serialize};

/// The APIKey definition of DYDX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub config: Str,
    pub mnemonic: Str,
}
