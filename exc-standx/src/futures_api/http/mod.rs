pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://perps.standx.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);
    let header = request.headers_mut();
    header.insert("Authorization", format!("Bearer {}", key.jwt).parse()?);

    let mut uri = format!("{}{}", host, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesApi)?;
        header.insert("x-request-sign-version", "v1".parse()?);
        header.insert("x-request-id", signature.signing.id.into());
        header.insert("x-request-timestamp", signature.signing.timestamp.into());
        header.insert("x-request-signature", signature.signature.parse()?);
    }

    let body = match req.method() {
        Method::GET | Method::DELETE => {
            uri.push('?');
            uri.push_str(&serde_urlencoded::to_string(req)?);
            header.insert("content-type", "application/x-www-form-urlencoded".try_into()?);
            String::new()
        }
        _ => {
            header.insert("content-type", "application/json".try_into()?);
            serde_json::to_string(req)?
        }
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(Body::wrap(body));
    Ok(request)
}
