pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://fx-api.gateio.ws";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("KEY", key.api_key.as_str().try_into()?);
    header.insert("content-type", "application/json".try_into()?);
    let mut uri = format!("{}{}", HOST, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::FuturesApi)?;

        header.insert("Timestamp", signature.signing.timestamp.into());
        header.insert("SIGN", signature.signature.try_into()?);
    }
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
