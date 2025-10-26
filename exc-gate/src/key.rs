use exc_core::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use hmac::{Hmac, Mac};
use http::Method;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use time::OffsetDateTime;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Gate.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub api_key: Str,
    pub secret_key: Str,
}

impl Key {
    /// Create a new [`Key`].
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: Str::new(api_key),
            secret_key: Str::new(secret_key),
        }
    }
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
        let now = OffsetDateTime::now_utc().unix_timestamp();
        Self::with_timestamp(params, now)
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
                if matches!(self.params.method(), Method::GET | Method::DELETE) {
                    let body = serde_urlencoded::to_string(self.params)?;
                    format!("{}\n{}\n{}\ncf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e\n{}", self.params.method().as_str(), self.params.path(), body, self.timestamp)
                } else {
                    let mut hasher = Sha512::new();
                    hasher.update(serde_json::to_string(self.params)?);
                    let body = hex::encode(hasher.finalize());
                    format!(
                        "{}\n{}\n\n{}\n{}",
                        self.params.method().as_str(),
                        self.params.path(),
                        body,
                        self.timestamp
                    )
                }
            }
            _ => {
                unreachable!()
            }
        };
        let signature = match kind {
            ApiKind::FuturesApi | ApiKind::SpotApi => {
                let mut mac = Hmac::<Sha512>::new_from_slice(key.secret_key.as_bytes())?;
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                hex::encode(mac_result.into_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
