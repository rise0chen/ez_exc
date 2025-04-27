pub mod book;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://www.okx.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".try_into()?);

    let mut uri = format!("{}{}", HOST, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::Common)?;

        header.insert("OK-ACCESS-KEY", key.api_key.as_str().try_into()?);
        header.insert("OK-ACCESS-SIGN", signature.signature.try_into()?);
        header.insert("OK-ACCESS-TIMESTAMP", signature.signing.timestamp.try_into()?);
        header.insert("OK-ACCESS-PASSPHRASE", key.passphrase.as_str().try_into()?);
    }
    let body = match req.method() {
        Method::GET => {
            uri.push('?');
            uri.push_str(&serde_urlencoded::to_string(req)?);
            Body::wrap(String::new())
        }
        _ => Body::wrap(serde_json::to_string(req)?),
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(body);
    Ok(request)
}
