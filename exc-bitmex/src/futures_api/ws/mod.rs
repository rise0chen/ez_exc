pub mod book;

use core::time::Duration;
use exc_util::types::book::{Depth, Order};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::vec;
use time::OffsetDateTime;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::Message;

const HOST: &str = "wss://ws.bitmex.com/realtime";

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
#[serde(rename_all = "camelCase")]
pub enum TxRequest {
    Ping,
    Pong,
    Subscribe(Vec<String>),
    Unsubscribe(Vec<String>),
    CancelAllAfter(u64),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "table")]
#[serde(rename_all = "camelCase")]
pub enum RxResponseData {
    OrderBookL2_25(book::GetOrdersResponse),
    OrderBook10(book::GetDepthResponse),
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
            let req_price = TxRequest::Subscribe(vec![format!("orderBook10:{s}")]);
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
                    if matches!(e, TxRequest::Pong) {
                        last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
                    } else {
                        tracing::warn!(e = ?e, "Unhandled Event");
                    }
                }
                RxResponse::Data(d) => match d {
                    RxResponseData::OrderBook10(d) => {
                        let d = d.data.first().unwrap();
                        let symbol = &d.symbol;
                        if let Some(ch) = self.books.get(symbol) {
                            ch.send_replace(Depth {
                                bid: d.bids.iter().map(|x| Order::new(x.0, x.1)).collect(),
                                ask: d.asks.iter().map(|x| Order::new(x.0, x.1)).collect(),
                                version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
                            });
                        } else {
                            tracing::warn!("Not init {}", symbol);
                        }
                    }
                    RxResponseData::OrderBookL2_25(d) => {
                        let symbol = d.data[0].symbol.clone();
                        if let Some(ch) = self.books.get(&symbol) {
                            let mut bid = Vec::new();
                            let mut ask = Vec::new();
                            for x in d.data {
                                if x.side.is_buy() {
                                    bid.push(Order::new(x.price, x.size as f64));
                                } else {
                                    ask.push(Order::new(x.price, x.size as f64));
                                }
                            }
                            bid.sort_by(|a, b| b.price.total_cmp(&a.price));
                            ask.sort_by(|a, b| a.price.total_cmp(&b.price));
                            let version = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64;
                            ch.send_replace(Depth { bid, ask, version });
                        } else {
                            tracing::warn!("Not init {}", symbol);
                        }
                    }
                },
            }
        }
    }
}
