pub use http::Method;
use serde::{Deserialize, Serialize};

pub enum ApiKind {
    Common,
    SpotApi,
    SpotWeb,
    FuturesApi,
    FuturesWeb,
}

pub trait Rest: Serialize {
    type Response: for<'de> Deserialize<'de> + Send + 'static;

    fn api_kind(&self) -> ApiKind;

    /// Get request method.
    fn method(&self) -> Method;

    /// Whether amend host.
    fn host(&self) -> Option<&'static str> {
        None
    }

    /// Get request path.
    fn path(&self) -> String;

    /// Whether need sign.
    fn need_sign(&self) -> bool {
        false
    }
}
