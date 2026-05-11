use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{eip712_domain, sol, Eip712Domain, SolStruct};
use exc_util::asset::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use http::Method;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use time::OffsetDateTime;

sol! {
    struct Message {
        string msg;
    }
}
const DOMAIN: Eip712Domain = eip712_domain! {
    name: "AsterSignTransaction",
    version: "1",
    chain_id: 1666,
    verifying_contract: alloy_primitives::address!(
        "0000000000000000000000000000000000000000"
    ),
};

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub account: Str,
    pub api: Str,
    pub secret: Str,
}

impl Key {
    /// Create a new [`Key`].
    pub fn sign<'a, T: Rest>(&'a self, params: &'a T, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        self.params(params).signed(self, format, kind)
    }
    pub fn params<'a, T: Rest>(&'a self, params: &'a T) -> SigningParams<'a, T> {
        let now = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1000;
        SigningParams {
            params,
            nonce: now,
            user: &self.account,
            signer: &self.api,
        }
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningParams<'a, T: Rest> {
    #[serde(flatten)]
    pub params: &'a T,
    pub nonce: i128,
    pub user: &'a str,
    pub signer: &'a str,
}

/// Signed params.
#[derive(Debug, Clone, Serialize)]
pub struct SignedParams<'a, T: Rest> {
    #[serde(flatten)]
    pub signing: SigningParams<'a, T>,
    pub signature: String,
}

impl<'a, T: Rest> SigningParams<'a, T> {
    /// Get signed params.
    pub fn signed(self, key: &Key, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        let body = match format {
            ParamsFormat::Common => {
                if matches!(self.params.method(), Method::GET | Method::DELETE) {
                    serde_urlencoded::to_string(&self)?
                } else {
                    serde_json::to_string(&self)?
                }
            }
            ParamsFormat::Json => serde_json::to_string(&self)?,
            ParamsFormat::Urlencoded => serde_urlencoded::to_string(&self)?,
        };
        let signer = PrivateKeySigner::from_str(&key.secret).unwrap();
        let signature = match kind {
            ApiKind::SpotApi => {
                todo!();
            }
            ApiKind::FuturesApi => {
                let raw = body;
                let msg = Message { msg: raw };
                let hash = msg.eip712_signing_hash(&DOMAIN);
                let result = signer.sign_hash_sync(&hash).unwrap();
                hex::encode(result.as_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
