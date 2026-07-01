pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::Key;
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://api.starknet.extended.exchange";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);
    let header = request.headers_mut();
    header.insert("user-agent", exc_util::constant::UA.try_into()?);
    header.insert("X-Api-Key", key.api_key.parse()?);

    let mut uri = format!("{}{}", host, req.path());
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            uri.push('?');
            uri.push_str(&serde_urlencoded::to_string(req)?);
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
