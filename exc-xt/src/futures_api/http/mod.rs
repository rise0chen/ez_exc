pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://fapi.xt.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();

    let mut uri = format!("{}{}", HOST, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::FuturesApi)?;

        header.insert("validate-appkey", key.api_key.as_str().try_into()?);
        header.insert("validate-signature", signature.signature.try_into()?);
        header.insert("validate-timestamp", signature.signing.timestamp.into());
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
