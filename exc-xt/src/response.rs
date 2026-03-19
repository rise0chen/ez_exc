use exc_core::ExchangeError;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// Code.
    pub code: String,
    /// Message.
    pub msg: Option<String>,
    /// Data.
    pub args: Value,
}

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    #[serde(alias = "rc")]
    pub return_code: i64,
    /// Message.
    pub error: Option<Error>,
    /// Data.
    pub result: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.return_code == 0 || value.return_code == 200 {
            value.result.ok_or(ExchangeError::UnexpectedResponseType(String::from("None")))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{}]: {:?}",
                value.return_code,
                value.error.unwrap_or_default()
            )))
        }
    }
}
