use crate::error::ExchangeError;
use futures_util::{future::BoxFuture, FutureExt as _, TryFutureExt as _};
use reqwest::Certificate;
pub use reqwest::{Body, Request, Response};

/// Http CLient
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
}

impl Client {
    /// Create a new websocket connector.
    pub fn new(cert: Option<&[u8]>) -> Self {
        let mut client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());
        if let Some(cert) = cert {
            client = client.tls_certs_merge(Certificate::from_pem(cert));
        }
        Self {
            inner: client.build().unwrap(),
        }
    }
}
impl tower::Service<Request> for Client {
    type Response = Response;
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(ExchangeError::Http)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        tower::Service::call(&mut self.inner, req).map_err(ExchangeError::Http).boxed()
    }
}
