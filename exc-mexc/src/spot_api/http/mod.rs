pub mod account;
pub mod book;
pub mod ping;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://api.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".try_into()?);
    header.insert("X-MEXC-APIKEY", key.api_key.as_str().try_into()?);
    let mut uri = format!("{}{}", HOST, req.path());
    let body_str = if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::SpotApi)?;
        serde_urlencoded::to_string(signature)?
    } else {
        serde_urlencoded::to_string(req)?
    };
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            uri.push('?');
            uri.push_str(&body_str);
            Body::wrap(String::new())
        }
        _ => Body::wrap(body_str),
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(body);
    Ok(request)
}
