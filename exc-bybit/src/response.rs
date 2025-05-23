use exc_core::ExchangeError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub list: Vec<T>,
}

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    pub ret_code: i64,
    /// Message.
    pub ret_msg: Option<String>,
    /// Data.
    pub result: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.ret_code == 0 {
            value.result.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!(
                "[{}]: {}",
                value.ret_code,
                value.ret_msg.unwrap_or_default()
            )))
        }
    }
}
