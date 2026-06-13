use exc_util::error::ExchangeError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub name: String,
    pub message: String,
}
/// HTTP API Response (with `code` and `msg`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FullHttpResponse<T> {
    Err { error: Error },
    Ok(T),
}
impl<T> From<FullHttpResponse<T>> for Result<T, ExchangeError> {
    fn from(value: FullHttpResponse<T>) -> Self {
        match value {
            FullHttpResponse::Ok(data) => Ok(data),
            FullHttpResponse::Err { error } => Err(ExchangeError::Api(anyhow::anyhow!("[{}]: {}", error.name, error.message))),
        }
    }
}
