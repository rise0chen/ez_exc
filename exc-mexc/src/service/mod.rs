mod book;
mod trading;

use crate::key::Key;
use crate::response::FullHttpResponse;
use exc_core::transport::http::{channel::HttpsChannel, endpoint::Endpoint as HttpsEndpoint};
use exc_core::ExchangeError;
use exc_util::interface::{ApiKind, Rest};
use futures::future::{ready, BoxFuture};
use futures::{FutureExt, TryFutureExt};
use http_body_util::BodyExt;
use tower::{Service, ServiceBuilder};
use tower_http::compression::{Compression, CompressionLayer};
use tower_http::decompression::{Decompression, DecompressionLayer};

/// Mexc API.
#[derive(Clone)]
pub struct Mexc {
    key: Key,
    http: Compression<Decompression<HttpsChannel>>,
}

impl Mexc {
    pub fn new(key: Key) -> Self {
        let http = ServiceBuilder::default()
            .layer(CompressionLayer::new().gzip(true))
            .layer(DecompressionLayer::new().gzip(true))
            .service(HttpsEndpoint::default().connect_https());
        Self { key, http }
    }
}

impl<Req: Rest> Service<Req> for Mexc {
    type Response = Req::Response;
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.http.poll_ready(cx).map_err(ExchangeError::from)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let req = match req.api_kind() {
            ApiKind::SpotApi => crate::spot_api::http::req_to_http(&req, &self.key),
            ApiKind::SpotWeb => crate::spot_web::http::req_to_http(&req, &self.key),
            ApiKind::FuturesApi => crate::futures_api::http::req_to_http(&req, &self.key),
            ApiKind::FuturesWeb => crate::futures_web::http::req_to_http(&req, &self.key),
            _ => unreachable!(),
        };
        match req {
            Ok(req) => self
                .http
                .call(req)
                .map_err(ExchangeError::from)
                .and_then(|resp| {
                    trace!("http response; status: {:?}", resp.status());
                    resp.into_body()
                        .collect()
                        .map(|x| x.map(|x| x.to_bytes()))
                        .map_err(|err| ExchangeError::UnexpectedResponseType(err.to_string()))
                })
                .and_then(|bytes| {
                    tracing::trace!(?bytes, "http response;");
                    let resp = match serde_json::from_slice::<FullHttpResponse<Req::Response>>(&bytes) {
                        Ok(res) => res.into(),
                        Err(_) => serde_json::from_slice::<Req::Response>(&bytes)
                            .map_err(|_| ExchangeError::UnexpectedResponseType(String::from_utf8_lossy(&bytes).into_owned())),
                        //.map_err(|e| ExchangeError::Other(e.into())),
                    };
                    ready(resp)
                })
                .boxed(),
            Err(err) => ready(Err(err.into())).boxed(),
        }
    }
}
