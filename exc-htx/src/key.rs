use base64::Engine as _;
use exc_core::Str;
pub use exc_util::interface::ApiKind;
use exc_util::interface::Rest;
use hmac::{Hmac, Mac};
use http::Method;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::format_description::well_known::iso8601::{Config, EncodedConfig, FormattedComponents, Iso8601, TimePrecision};
use time::OffsetDateTime;

const HOST_FUTURES: &str = "https://api.hbdm.com";
const HOST_SPOT: &str = "https://api.huobi.pro";
const TIME_CFG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second { decimal_digits: None })
    .set_formatted_components(FormattedComponents::DateTime)
    .encode();
type TimeFmt = Iso8601<TIME_CFG>;

pub enum ParamsFormat {
    Common,
    Json,
    Urlencoded,
}

/// The APIKey definition of Htx.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    pub account_id: u64,
    pub api_key: Str,
    pub secret_key: Str,
}

impl Key {
    /// Create a new [`Key`].
    pub fn new(account_id: u64, api_key: &str, secret_key: &str) -> Self {
        Self {
            account_id,
            api_key: Str::new(api_key),
            secret_key: Str::new(secret_key),
        }
    }
    pub fn sign<'a, T: Rest>(&'a self, params: &'a T, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        SigningParams::now(params, &self.api_key).signed(self, format, kind)
    }
}

/// Signing params.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SigningParams<'a, T: Rest> {
    pub access_key_id: &'a str,
    pub signature_method: &'static str,
    pub signature_version: u8,
    pub timestamp: String,
    #[serde(flatten)]
    pub params: Option<&'a T>,
}

impl<'a, T: Rest> SigningParams<'a, T> {
    fn with_timestamp(params: &'a T, api_key: &'a str, timestamp: String) -> Self {
        Self {
            access_key_id: api_key,
            signature_method: "HmacSHA256",
            signature_version: 2,
            timestamp,
            params: Some(params),
        }
    }

    /// Sign the given params now.
    pub fn now(params: &'a T, api_key: &'a str) -> Self {
        let now = OffsetDateTime::now_utc();
        let now = now.replace_millisecond(0).unwrap();
        Self::with_timestamp(params, api_key, now.format(&TimeFmt {}).unwrap())
    }
}

/// Signed params.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignedParams<'a, T: Rest> {
    #[serde(flatten)]
    pub signing: SigningParams<'a, T>,
    pub signature: String,
}

impl<'a, T: Rest> SigningParams<'a, T> {
    /// Get signed params.
    pub fn signed(mut self, key: &Key, format: ParamsFormat, kind: ApiKind) -> Result<SignedParams<'a, T>, anyhow::Error> {
        let params = self.params.unwrap();
        let raw = match format {
            ParamsFormat::Common => {
                let body = if matches!(params.method(), Method::GET | Method::DELETE) {
                    serde_urlencoded::to_string(&self)?
                } else {
                    self.params = None;
                    serde_urlencoded::to_string(&self)?
                };
                let host = params
                    .host()
                    .unwrap_or(match kind {
                        ApiKind::Common => todo!(),
                        ApiKind::SpotApi => HOST_SPOT,
                        ApiKind::SpotWeb => todo!(),
                        ApiKind::FuturesApi => HOST_FUTURES,
                        ApiKind::FuturesWeb => todo!(),
                    })
                    .strip_prefix("https://")
                    .unwrap_or_default();
                format!("{}\n{}\n{}\n{}", params.method().as_str(), host, params.path(), body)
            }
            _ => {
                unreachable!()
            }
        };
        let signature = match kind {
            ApiKind::FuturesApi | ApiKind::SpotApi => {
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
