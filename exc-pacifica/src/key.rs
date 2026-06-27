use exc_util::asset::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use http::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

fn req_type(uri: &str) -> &'static str {
    match uri {
        "/api/v1/orders/create" => "create_order",
        "/api/v1/orders/cancel" => "cancel_order",
        _ => "",
    }
}

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub account: Str,
    pub agent: Str,
    pub secret: Str,
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
            expiry_window: 5000,
            timestamp: now,
            r#type: req_type(&params.path()),
        }
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SigningParams<'a, T: Rest> {
    #[serde(rename = "data")]
    pub params: &'a T,
    pub expiry_window: u64,
    pub timestamp: u64,
    pub r#type: &'static str,
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
        let secret = bs58::decode(&key.secret).into_vec().unwrap();
        let signer = ed25519_consensus::SigningKey::try_from(&secret[..32]).unwrap();
        let signature = match kind {
            ApiKind::SpotApi => {
                todo!();
            }
            ApiKind::FuturesApi => {
                let result = signer.sign(body.as_bytes());
                bs58::encode(result.to_bytes()).into_string()
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
