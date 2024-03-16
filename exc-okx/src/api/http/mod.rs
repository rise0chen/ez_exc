pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::interface::{Method, Rest};
use http::Request;
use hyper::Body;

const HOST: &str = "https://aws.okx.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut uri = format!("{}{}", HOST, req.path());
    let signature = if req.need_sign() {
        Some(key.sign(req, ParamsFormat::Common, ApiKind::Common)?)
    } else {
        None
    };
    let body = match req.method() {
        Method::GET => {
            uri.push('?');
            uri.push_str(&serde_urlencoded::to_string(req)?);
            hyper::Body::empty()
        }
        _ => hyper::Body::from(serde_json::to_string(req)?),
    };

    let builder = Request::builder()
        .method(req.method())
        .uri(uri)
        .header("content-type", "application/json");
    let builder = if let Some(signature) = signature {
        builder
            .header("OK-ACCESS-KEY", &*key.api_key)
            .header("OK-ACCESS-SIGN", signature.signature)
            .header("OK-ACCESS-TIMESTAMP", signature.signing.timestamp)
            .header("OK-ACCESS-PASSPHRASE", &*key.passphrase)
    } else {
        builder
    };
    let request = builder.body(body)?;
    Ok(request)
}
