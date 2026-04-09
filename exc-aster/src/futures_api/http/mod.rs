pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://fapi.asterdex.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/x-www-form-urlencoded".try_into()?);
    header.insert("Accept-Encoding", "gzip".parse()?);
    header.insert(
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0".try_into()?,
    );

    let mut uri = format!("{}{}", HOST, req.path());
    let body = if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::FuturesApi)?;

        header.insert("X-MBX-APIKEY", key.api_key.as_str().try_into()?);
        serde_urlencoded::to_string(signature)?
    } else {
        serde_urlencoded::to_string(req)?
    };
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            uri.push('?');
            uri.push_str(&body);
            String::new()
        }
        _ => body,
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(Body::wrap(body));
    Ok(request)
}
