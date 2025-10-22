use exc_core::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Binance.
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
    pub fn sign<'a, T: Rest>(
        &self,
        params: &'a T,
        format: ParamsFormat,
        kind: ApiKind,
    ) -> Result<SignedParams<'a, T>, anyhow::Error> {
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
            ParamsFormat::Common => serde_urlencoded::to_string(&self)?,
            _ => {
                unreachable!()
            }
        };
        let signature = match kind {
            ApiKind::FuturesApi | ApiKind::SpotApi => {
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
