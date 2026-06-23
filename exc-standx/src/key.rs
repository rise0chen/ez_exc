use base64::Engine as _;
use exc_util::asset::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use http::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub secret: Str,
    pub jwt: Str,
}

impl Key {
    /// Create a new [`Key`].
    pub fn sign<'a, T: Rest>(&'a self, params: &'a T, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        self.params(params).signed(self, format, kind)
    }
    pub fn params<'a, T: Rest>(&'a self, params: &'a T) -> SigningParams<'a, T> {
        let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
        SigningParams {
            params,
            id: now,
            timestamp: now,
        }
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SigningParams<'a, T: Rest> {
    #[serde(flatten)]
    pub params: &'a T,
    pub id: u64,
    pub timestamp: u64,
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
                    serde_urlencoded::to_string(self.params)?
                } else {
                    serde_json::to_string(&self.params)?
                }
            }
            ParamsFormat::Json => serde_json::to_string(&self.params)?,
            ParamsFormat::Urlencoded => serde_urlencoded::to_string(self.params)?,
        };
        let secret = bs58::decode(&key.secret).into_vec().unwrap();
        let signer = ed25519_consensus::SigningKey::try_from(secret.as_slice()).unwrap();
        let signature = match kind {
            ApiKind::SpotApi => {
                todo!();
            }
            ApiKind::FuturesApi => {
                let raw = format!("v1,{},{},{}", self.id, self.timestamp, body);
                let result = signer.sign(raw.as_bytes());
                base64::engine::general_purpose::STANDARD.encode(result.to_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
