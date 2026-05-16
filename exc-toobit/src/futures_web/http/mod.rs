pub mod trading;

use crate::key::Key;
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};
use serde::Serialize;

const HOST: &str = "https://bapi.toobit.com";

#[derive(Debug, Clone, Serialize)]
pub struct Query<'a, T: Rest> {
    #[serde(flatten)]
    pub params: &'a T,
    pub c_token: &'a str,
}

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);

    let mut uri = format!("{}{}", host, req.path());
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
    let header = request.headers_mut();
    header.insert("content-type", "application/x-www-form-urlencoded".try_into()?);
    if let Some(key) = &key.web_key {
        header.insert("cookie", format!("user_id={}; au_token={}", key.user_id, key.au_token).try_into()?);
        request.url_mut().query_pairs_mut().append_pair("c_token", &key.c_token);
    }
    request.body_mut().replace(Body::wrap(body));
    Ok(request)
}
