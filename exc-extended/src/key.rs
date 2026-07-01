use exc_util::asset::Str;
pub use exc_util::interface::ApiKind;
use serde::{Deserialize, Serialize};
use starknet_core::{crypto::Signature, types::Felt};

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub vault: u32,
    pub api_key: Str,
    pub public: Str,
    pub secret: Str,
}

impl Key {
    /// Create a new [`Key`].
    pub fn sign(&self, hash: Felt) -> Result<Signature, anyhow::Error> {
        let secret = Felt::from_hex(&self.secret).unwrap();
        let signer = starknet_signers::SigningKey::from_secret_scalar(secret);
        Ok(signer.sign(&hash)?)
    }
}
