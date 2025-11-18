use exc_core::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
pub struct FullHttpResponse<T> {
    pub status: Option<String>,
    /// Code.
    pub code: Option<i64>,
    /// Message.
    #[serde(alias = "message")]
    #[serde(alias = "err-msg")]
    pub msg: Option<String>,
    /// Data.
    #[serde(flatten)]
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code == Some(0) || value.code == Some(200) || value.status.as_deref() == Some("ok") || value.status.as_deref() == Some("success") {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{}]: {}",
                value.status.unwrap_or(value.code.unwrap_or(-1).to_string()),
                value.msg.unwrap_or_default()
            )))
        }
    }
}
