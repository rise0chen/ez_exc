use exc_util::error::ExchangeError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data<T> {
    pub data: T,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub result_list: Vec<T>,
}

/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullHttpResponse<T> {
    /// Code.
    #[serde(alias = "status", alias = "error_code")]
    pub code: i32,
    /// Message.
    #[serde(alias = "message")]
    pub msg: String,
    /// Data.
    #[serde(alias = "dataWrapper")]
    pub data: Option<T>,
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        if value.code == 0 || value.code == 200 {
            value.data.ok_or(ExchangeError::UnexpectedResponseType(String::new()))
        } else {
            Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", value.code, value.msg)))
        }
    }
}
