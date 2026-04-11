pub mod info;

use crate::key::Key;
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://api.bitunix.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".try_into()?);
    header.insert(
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 Edg/140.0.0.0".try_into()?,
    );
    header.insert("exchange-client", "pc".try_into()?);
    header.insert("exchange-language", "zh_TW".try_into()?);
    if req.need_sign() {
        if let Some(token) = &key.web_key {
            header.insert("exchange-token", token.as_str().try_into()?);
            header.insert("token", token.as_str().try_into()?);
        }
    }
    let mut uri = format!("{}{}", host, req.path());
    let body = match req.method() {
        Method::GET | Method::DELETE => {
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
