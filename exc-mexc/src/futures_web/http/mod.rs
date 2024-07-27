pub mod trading;

use crate::key::{ApiKind, Key, ParamsFormat};
use exc_util::interface::{Method, Rest};
use http::Request;
use hyper::Body;

const HOST: &str = "https://futures.mexc.com";

pub fn req_to_http<Req: Rest>(req: &Req, key: &Key) -> Result<Request<Body>, anyhow::Error> {
    let mut builder = Request::builder().method(req.method()).header("content-type", "application/json");
    if req.need_sign() {
        if let Some(token) = &key.web_key {
            builder = builder
                .header("accept", "*/*")
                .header("accept-language", "en-US,en;q=0.9")
                .header("origin", HOST)
                .header("sec-ch-ua", r#"Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126"#)
                .header("sec-fetch-mode", "cors")
                .header("sec-fetch-site", "same-origin")
                .header("sec-fetch-user", "?1")
                .header(
                    "user-agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
                )
                .header("cookie", format!("uc_token={}; u_id={}", token, token))
                .header("Authorization", token.as_str());
        }
    }
    let mut uri = format!("{}{}", HOST, req.path());
    let body = match req.method() {
        Method::GET => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Urlencoded, ApiKind::FuturesWeb)?;
                builder = builder
                    .header("x-mxc-nonce", signature.signing.timestamp)
                    .header("x-mxc-sign", &signature.signature);
                serde_urlencoded::to_string(signature.signing)?
            } else {
                serde_urlencoded::to_string(req)?
            };
            uri.push('?');
            uri.push_str(&body_str);
            hyper::Body::empty()
        }
        _ => {
            let body_str = if req.need_sign() {
                let signature = key.sign(req, ParamsFormat::Json, ApiKind::FuturesWeb)?;
                builder = builder
                    .header("x-mxc-nonce", signature.signing.timestamp)
                    .header("x-mxc-sign", &signature.signature);
                serde_json::to_string(&signature.signing)?
            } else {
                serde_json::to_string(req)?
            };
            hyper::Body::from(body_str)
        }
    };

    let request = builder.uri(uri).body(body)?;
    Ok(request)
}
