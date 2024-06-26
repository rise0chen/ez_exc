pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::interface::{Method, Rest};
use http::Request;
use hyper::Body;

const HOST: &str = "https://fx-api.gateio.ws";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut builder = Request::builder()
        .method(req.method())
        .header("KEY", key.api_key.as_str())
        .header("content-type", "application/json");
    let mut uri = format!("{}{}", HOST, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::FuturesApi)?;
        builder = builder
            .header("Timestamp", signature.signing.timestamp)
            .header("SIGN", &signature.signature);
    }
    let body = match req.method() {
        Method::GET => {
            let body_str = serde_urlencoded::to_string(req)?;
            uri.push('?');
            uri.push_str(&body_str);
            hyper::Body::empty()
        }
        _ => {
            let body_str = serde_json::to_string(req)?;
            hyper::Body::from(body_str)
        }
    };

    let request = builder.uri(uri).body(body)?;
    Ok(request)
}
