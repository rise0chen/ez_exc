use exc_util::error::ExchangeError;
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
    pub d: Option<T>,
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code.is_none() || value.code == Some(0) || value.code == Some(200) {
            value.data.or(value.d).ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{}]: {}",
                value.code.unwrap_or_default(),
                value.msg.unwrap_or_default()
            )))
        }
    }
}
