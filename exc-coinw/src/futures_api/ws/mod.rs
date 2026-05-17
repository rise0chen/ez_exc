pub mod book;

use core::time::Duration;
use exc_util::types::book::{Depth, Order};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::Message;

const HOST: &str = "wss://ws.futurescw.com/perpum";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Ping,
    Pong,
    Sub,
    Unsub,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxRequestParams {
    pub r#type: String,
    pub biz: String,
    pub pair_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TxRequest {
    pub event: Event,
    pub params: Option<TxRequestParams>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RxResponseData {
    Depth(book::GetDepthResponse),
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RxResponse {
    Data(RxResponseData),
    Event(TxRequest),
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
            let req_price = TxRequest {
                event: Event::Sub,
                params: Some(TxRequestParams {
                    r#type: "depth".into(),
                    biz: "futures".into(),
                    pair_code: s.into(),
                }),
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
                    let req_ping = TxRequest {
                        event: Event::Ping,
                        params: None,
                    };
                    write.send(Message::Text(serde_json::to_string(&req_ping)?.into())).await?;
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

            let text = if let Message::Text(text) = message {
                (*text).into()
            } else if let Message::Ping(m) = message {
                write.send(Message::Pong(m)).await?;
                continue;
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
            let m: RxResponse = match serde_json::from_str(&text) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!(e = ?e, text = ?text, "Unhandled message");
                    continue;
                }
            };
            match m {
                RxResponse::Event(e) => {
                    if matches!(e.event, Event::Pong) {
                        last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
                    } else {
                        tracing::warn!(e = ?e, "Unhandled Event");
                    }
                }
                RxResponse::Data(d) => match d {
                    RxResponseData::Depth(d) => {
                        let symbol = d.pair_code;
                        if let Some(ch) = self.books.get(&symbol) {
                            ch.send_replace(Depth {
                                bid: d.data.bids.iter().map(|x| Order::new(x.p, x.m)).collect(),
                                ask: d.data.asks.iter().map(|x| Order::new(x.p, x.m)).collect(),
                                version: d.data.t,
                            });
                        } else {
                            tracing::warn!("Not init {}", symbol);
                        }
                    }
                },
            }
        }
    }
}
