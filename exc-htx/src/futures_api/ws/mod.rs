pub mod book;

use core::time::Duration;
use exc_util::types::book::{Depth, Order};
use flate2::read::GzDecoder;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read as _, sync::Arc};
use time::OffsetDateTime;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::Message;

const HOST: &str = "wss://api.hbdm.vn/linear-swap-ws";

#[derive(Debug, Serialize)]
pub struct Tx {
    pub sub: String,
    pub id: String,
}
#[derive(Debug, Serialize)]
pub struct Pong {
    pub pong: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RxResponseData {
    Book(book::GetDepthResponse),
}
#[derive(Debug, Deserialize)]
pub struct RxResponse {
    pub ch: String,
    pub ts: u64,
    #[serde(flatten)]
    pub data: RxResponseData,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RxRequest {
    Ping { ping: u64 },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Rx {
    Request(RxRequest),
    Response(RxResponse),
}

#[derive(Clone)]
pub struct Ws {
    pub symbols: Vec<String>,
    pub books: Arc<HashMap<String, watch::Sender<Depth>>>,
}
impl Ws {
    pub fn new(symbols: Vec<String>) -> Ws {
        let books = Arc::new(symbols.iter().map(|s| (s.clone(), watch::channel(Depth::default()).0)).collect());
        Ws { symbols, books }
    }
    pub fn clear(&self) {
        self.books.values().for_each(|x| {
            x.send_replace(Depth::default());
        });
    }
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(HOST).await?;
        tracing::info!(base_url = HOST, "WebSocket connected");
        let (mut write, mut read) = ws_stream.split();
        for s in &self.symbols {
            let req_price = Tx {
                sub: format!("market.{}.depth.step6", s),
                id: format!("depth_{}", s),
            };
            write.send(Message::Text(serde_json::to_string(&req_price)?.into())).await?;
        }

        // Message handling loop
        let mut last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
        let mut interval = tokio::time::interval(Duration::from_secs(8));
        loop {
            let message = match futures_util::future::select(Box::pin(interval.tick()), read.next()).await {
                futures_util::future::Either::Left(_) => {
                    let now = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
                    if now - last_time > 30 {
                        return Ok(());
                    }
                    write.send(Message::Ping("".into())).await?;
                    continue;
                }
                futures_util::future::Either::Right((m, _)) => {
                    if let Some(m) = m {
                        m?
                    } else {
                        return Ok(());
                    }
                }
            };

            let text = if let Message::Binary(text) = message {
                let mut decoder = GzDecoder::new(&text[..]);
                let mut decompressed = Vec::new();

                match decoder.read_to_end(&mut decompressed) {
                    Ok(_) => decompressed,
                    Err(e) => {
                        tracing::error!("gzip 解压失败: {}", e);
                        continue;
                    }
                }
            } else if let Message::Text(text) = message {
                (*text).into()
            } else if let Message::Pong(_) = message {
                last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
                continue;
            } else {
                tracing::warn!(message = ?message, "Unhandled ws message");
                continue;
            };
            let text = match String::from_utf8(text) {
                Ok(text) => text,
                Err(e) => {
                    tracing::warn!(e = ?e, "Invaild UTF8");
                    continue;
                }
            };
            let m: Rx = match serde_json::from_str(&text) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!(e = ?e, text = ?text, "Unhandled message");
                    continue;
                }
            };
            let m = match m {
                Rx::Request(request) => {
                    match request {
                        RxRequest::Ping { ping } => {
                            let req_ping = Pong { pong: ping };
                            write.send(Message::Text(serde_json::to_string(&req_ping)?.into())).await?;
                        }
                    }
                    continue;
                }
                Rx::Response(response) => response,
            };
            match m.data {
                RxResponseData::Book(d) => {
                    let symbol = m.ch.split('.').nth(1).unwrap();
                    if let Some(ch) = self.books.get(symbol) {
                        let resp = d.tick;
                        ch.send_replace(Depth {
                            bid: resp.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                            ask: resp.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                            version: resp.ts,
                        });
                    } else {
                        tracing::warn!("Not init {}", symbol);
                    }
                }
            }
        }
    }
}
