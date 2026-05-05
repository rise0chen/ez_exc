use exc_util::asset::Str;
use serde::{Deserialize, Serialize};

/// The APIKey definition of GRVT.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub account_id: Str,
    pub api_key: Str,
    pub secret_key: Str,
}
