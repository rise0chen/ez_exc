pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::Key;
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://mainnet.zklighter.elliot.ai";

pub fn req_to_http<Req: Rest>(req: &Req, _key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);

    let mut uri = format!("{}{}", HOST, req.path());
    let body = serde_urlencoded::to_string(req)?;
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
