use exc_core::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use http::Method;
use md5::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Bitget.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub api_key: Str,
    pub secret_key: Str,
    pub web_key: Option<Str>,
}

impl Key {
    /// Create a new [`Key`].
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: Str::new(api_key),
            secret_key: Str::new(secret_key),
            web_key: None,
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
        let body = match format {
            ParamsFormat::Common => {
                if matches!(self.params.method(), Method::GET | Method::DELETE) {
                    serde_urlencoded::to_string(self.params)?.replace(['=', '&'], "")
                } else {
                    serde_json::to_string(self.params)?
                }
            }
            ParamsFormat::Json => serde_json::to_string(self.params)?,
            ParamsFormat::Urlencoded => serde_urlencoded::to_string(self.params)?.replace(['=', '&'], ""),
        };
        let raw = format!("{}{}{}", self.timestamp, key.api_key, body);
        let signature = match kind {
            ApiKind::Common => {
                let mut mac = Sha256::new();
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                let mut mac = Sha256::new();
                mac.update(hex::encode(mac_result));
                mac.update(key.secret_key.as_str());
                let mac_result = mac.finalize();
                hex::encode(mac_result)
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
