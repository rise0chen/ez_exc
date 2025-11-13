pub mod account;
pub mod book;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://fapi.bitunix.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".try_into()?);
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::Common)?;
        header.insert("api-key", key.api_key.as_str().try_into()?);
        header.insert("nonce", "".try_into()?);
        header.insert("timestamp", signature.signing.timestamp.into());
        header.insert("sign", signature.signature.try_into()?);
    }
    let mut uri = format!("{}{}", HOST, req.path());
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            let body_str = serde_urlencoded::to_string(req)?;
            uri.push('?');
            uri.push_str(&body_str);
            Body::wrap(String::new())
        }
        _ => {
            let body_str = serde_json::to_string(req)?;
            Body::wrap(body_str)
        }
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(body);
    Ok(request)
}
