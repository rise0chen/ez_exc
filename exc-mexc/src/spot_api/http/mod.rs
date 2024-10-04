pub mod account;
pub mod book;
pub mod ping;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::channel::{Body, Bytes};
use exc_util::interface::{Method, Rest};
use http::Request;

const HOST: &str = "https://api.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut uri = format!("{}{}", HOST, req.path());
    let body_str = if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::SpotApi)?;
        serde_urlencoded::to_string(signature)?
    } else {
        serde_urlencoded::to_string(req)?
    };
    let body = match req.method() {
        Method::GET => {
            uri.push('?');
            uri.push_str(&body_str);
            Body::new(Bytes::new())
        }
        _ => Body::new(body_str.into()),
    };

    let builder = Request::builder()
        .method(req.method())
        .uri(uri)
        .header("content-type", "application/json")
        .header("accept-encoding", "gzip")
        .header("X-MEXC-APIKEY", &*key.api_key);
    let request = builder.body(body)?;
    Ok(request)
}
