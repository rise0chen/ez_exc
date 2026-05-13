use exc_util::error::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    pub code: i32,
    /// Message.
    pub msg: String,
    /// Data.
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code == 0 {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", value.code, value.msg)))
        }
    }
}
