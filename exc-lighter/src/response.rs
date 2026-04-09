use exc_util::error::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FullHttpResponse<T> {
    /// Code.
    pub code: i64,
    /// Message.
    pub message: Option<String>,
    /// Data.
    #[serde(flatten)]
    pub result: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code == 0 || value.code == 200 {
            value.result.ok_or(ExchangeError::UnexpectedResponseType(String::from("None")))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{}]: {:?}",
                value.code,
                value.message.unwrap_or_default()
            )))
        }
    }
}
