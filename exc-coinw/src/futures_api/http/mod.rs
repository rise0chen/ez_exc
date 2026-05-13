pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://api.coinw.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".parse()?);
    header.insert("user-agent", exc_util::constant::UA.try_into()?);
    header.insert("deviceid", "b81d84094c28be90999dd916fc2e7817".parse()?);
    let mut uri = format!("{}{}", host, req.path());
    if req.need_sign() {
        let signature = key.sign(req, ParamsFormat::Common, ApiKind::FuturesApi)?;

        header.insert("api_key", key.api_key.as_str().parse()?);
        header.insert("timestamp", signature.signing.timestamp.into());
        header.insert("sign", signature.signature.try_into()?);
    }
    let body = match req.method() {
        Method::GET => {
            let body_str = serde_urlencoded::to_string(req)?;
            uri.push('?');
            uri.push_str(&body_str);
            String::new()
        }
        _ => serde_json::to_string(req)?,
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(Body::wrap(body));
    Ok(request)
}
