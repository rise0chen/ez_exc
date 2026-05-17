use base64::Engine as _;
use exc_util::asset::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use hmac::{Hmac, Mac};
use http::Method;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Coinw.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub api_key: Str,
    secret_key: Str,
    #[serde(default)]
    pub symbol: Str,
}

impl Key {
    pub fn sign<'a, T: Rest>(&self, params: &'a T, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        SigningParams::now(params).signed(self, format, kind)
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
pub struct SigningParams<'a, T: Rest> {
    #[serde(flatten)]
    pub params: &'a T,
    pub timestamp: i64,
}

impl<'a, T: Rest> SigningParams<'a, T> {
    fn with_timestamp(params: &'a T, timestamp: i64) -> Self {
        Self { params, timestamp }
    }

    /// Sign the given params now.
    pub fn now(params: &'a T) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000;
        Self::with_timestamp(params, now as i64)
    }
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
        let raw = match format {
            ParamsFormat::Common => {
                if matches!(self.params.method(), Method::GET) {
                    let body = serde_urlencoded::to_string(self.params)?;
                    if body.is_empty() {
                        format!("{}{}{}", self.timestamp, self.params.method(), self.params.path())
                    } else {
                        format!("{}{}{}?{}", self.timestamp, self.params.method(), self.params.path(), body)
                    }
                } else {
                    let body = serde_json::to_string(&self.params)?;
                    format!("{}{}{}{}", self.timestamp, self.params.method(), self.params.path(), body)
                }
            }
            _ => {
                unreachable!()
            }
        };
        let signature = match kind {
            ApiKind::FuturesApi => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.secret_key.as_bytes())?;
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                base64::engine::general_purpose::STANDARD.encode(mac_result.into_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
