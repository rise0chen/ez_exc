use exc_util::error::ExchangeError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// Code.
    pub code: i64,
    /// Message.
    pub message: String,
}

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    pub status: String,
    /// Message.
    pub error: Option<Error>,
    /// Data.
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if let Some(e) = value.error {
            Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", e.code, e.message)))
        } else {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        }
    }
}
