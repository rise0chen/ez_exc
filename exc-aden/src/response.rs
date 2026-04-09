use exc_util::error::ExchangeError;
use serde::Deserialize;

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    pub label: Option<String>,
    /// Message.
    pub message: Option<String>,
    /// Data.
    #[serde(flatten)]
    pub result: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.message.is_none() && value.label.is_none() {
            value.result.ok_or(ExchangeError::UnexpectedResponseType(String::from("None")))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{:?}]: {:?}",
                value.label,
                value.message.unwrap_or_default()
            )))
        }
    }
}
