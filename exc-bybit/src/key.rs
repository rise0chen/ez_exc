use exc_core::Str;
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

/// The APIKey definition of MEXC.
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
    pub fn sign<'a, T: Rest<Response = R>, R>(
        &self,
        params: &'a T,
        format: ParamsFormat,
        kind: ApiKind,
    ) -> Result<SignedParams<'a, T, R>, anyhow::Error> {
        SigningParams::now(params).signed(self, format, kind)
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
pub struct SigningParams<'a, T: Rest<Response = R>, R> {
    #[serde(flatten)]
    pub params: &'a T,
    pub timestamp: i64,
}

impl<'a, T: Rest<Response = R>, R> SigningParams<'a, T, R> {
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
pub struct SignedParams<'a, T: Rest<Response = R>, R> {
    #[serde(flatten)]
    pub signing: SigningParams<'a, T, R>,
    pub signature: String,
}

impl<'a, T: Rest<Response = R>, R> SigningParams<'a, T, R> {
    /// Get signed params.
    pub fn signed(self, key: &Key, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T, R>, anyhow::Error> {
        let body = match format {
            ParamsFormat::Common => {
                if matches!(self.params.method(), Method::GET | Method::DELETE) {
                    serde_urlencoded::to_string(self.params)?
                } else {
                    serde_json::to_string(self.params)?
                }
            }
            ParamsFormat::Json => serde_json::to_string(self.params)?,
            ParamsFormat::Urlencoded => serde_urlencoded::to_string(self.params)?,
        };
        let raw = format!("{}{}{}", self.timestamp, key.api_key, body);
        let signature = match kind {
            ApiKind::Common => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.secret_key.as_bytes())?;
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                hex::encode(mac_result.into_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
