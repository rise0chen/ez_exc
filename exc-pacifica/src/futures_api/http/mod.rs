pub mod account;
pub mod book;
pub mod info;
pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::http::{Body, Request};
use exc_util::interface::{Method, Rest};
use serde::Serialize;

const HOST: &str = "https://api.pacifica.fi";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct SignedBody<'a, T: Rest> {
    pub account: &'a str,
    pub agent_wallet: &'a str,
    pub expiry_window: u64,
    pub signature: &'a str,
    pub timestamp: u64,
    #[serde(flatten)]
    pub params: &'a T,
}

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let host = req.host().unwrap_or(HOST);
    let mut request = Request::new(req.method(), host.parse()?);
    let header = request.headers_mut();

    let mut uri = format!("{}{}", host, req.path());
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            uri.push('?');
            uri.push_str(&serde_urlencoded::to_string(req)?);
            String::new()
        }
        _ => {
            header.insert("content-type", "application/json".try_into()?);
            if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesApi)?;
                serde_json::to_string(&SignedBody {
                    account: &key.account,
                    agent_wallet: &key.agent,
                    expiry_window: signature.signing.expiry_window,
                    signature: &signature.signature,
                    timestamp: signature.signing.timestamp,
                    params: req,
                })?
            } else {
                serde_json::to_string(req)?
            }
        }
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(Body::wrap(body));
    Ok(request)
}
