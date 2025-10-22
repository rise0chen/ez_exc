use exc_core::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
pub struct FullHttpResponse<T> {
    /// Code.
    pub code: Option<i64>,
    /// Message.
    #[serde(alias = "message")]
    pub msg: Option<String>,
    /// Data.
    #[serde(flatten)]
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if let Some(code) = value.code {
            Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", code, value.msg.unwrap_or_default())))
        } else {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        }
    }
}
