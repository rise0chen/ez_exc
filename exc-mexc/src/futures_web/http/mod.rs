pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_core::transport::http::{Body, Request};
use exc_util::interface::{Method, Rest};

const HOST: &str = "https://futures.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request, anyhow::Error> {
    let mut request = Request::new(req.method(), HOST.parse()?);
    let header = request.headers_mut();
    header.insert("content-type", "application/json".try_into()?);
    header.insert("Accept-Encoding", "gzip".parse()?);
    header.insert(
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0".try_into()?,
    );
    if req.need_sign() {
        if let Some(token) = &key.web_key {
            header.insert("cookie", format!("uc_token={}; u_id={}", token, token).try_into()?);
            header.insert("Authorization", token.as_str().try_into()?);
        }
    }
    let mut uri = format!("{}{}", HOST, req.path());
    let body = match req.method() {
        Method::GET | Method::DELETE => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::FuturesWeb)?;
                header.insert("x-mxc-nonce", signature.signing.timestamp.into());
                header.insert("x-mxc-sign", signature.signature.try_into()?);
                serde_urlencoded::to_string(signature.signing.params)?
            } else {
                serde_urlencoded::to_string(req)?
            };
            uri.push('?');
            uri.push_str(&body_str);
            Body::wrap(String::new())
        }
        _ => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesWeb)?;
                header.insert("x-mxc-nonce", signature.signing.timestamp.into());
                header.insert("x-mxc-sign", signature.signature.try_into()?);
                serde_json::to_string(&signature.signing.params)?
            } else {
                serde_json::to_string(req)?
            };
            Body::wrap(body_str)
        }
    };

    *request.url_mut() = uri.parse()?;
    request.body_mut().replace(body);
    Ok(request)
}
