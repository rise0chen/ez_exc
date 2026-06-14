use exc_util::asset::Str;
pub use exc_util::constant::UA;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::OffsetDateTime;

pub const VERSION: i64 = 20251120;
const KEY: &str = "23bec4f84891096e112812c32c7c31b3";
pub const DEVICE_ID: &str = "lWihceCBhZirB5l9zC1c6WR1VuE0wQSZ";

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Lbank.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub web_key: Str,
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
        let timestamp = timestamp + 5_000;
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
    pub fn signed(self, _key: &Key, _format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        let raw = format!(
            "{}{}{}{}{}WEBWEB{}",
            self.params.method(),
            self.params.path(),
            self.timestamp,
            UA,
            VERSION,
            DEVICE_ID
        );
        let signature = match kind {
            ApiKind::FuturesWeb => {
                let mut mac = Hmac::<Sha256>::new_from_slice(KEY.as_bytes())?;
                mac.update(raw.as_bytes());
                let mac_result = mac.finalize();
                hex::encode(mac_result.into_bytes())
            }
            _ => unreachable!(),
        };
        Ok(SignedParams { signing: self, signature })
    }
}
