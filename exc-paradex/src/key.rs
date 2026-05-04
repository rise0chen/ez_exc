use exc_util::asset::Str;
use serde::{Deserialize, Serialize};

/// The APIKey definition of PARADEX.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub secret_key: Str,
    #[serde(default)]
    pub pro: bool,
}
