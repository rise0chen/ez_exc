pub mod book;
pub mod info;

use core::time::Duration;
use exc_util::types::book::{Depth, DepthManger, Order};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use std::collections::HashMap;
use time::OffsetDateTime;
use tokio::sync::{Mutex, watch};
use tokio_tungstenite::tungstenite::Message;

const HOST: &str = "wss://uuws.rerrkvifj.com/ws/v3";

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    i: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TxRequest {
    // 3订单簿
    x: i32,
    // 订阅ID
    #[serde_as(as = "DisplayFromStr")]
    y: usize,
    // 1订阅 0取消
    z: i32,
    e: &'static str,
    a: Channel,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RxResponseData {
    OrderBook(book::GetDepthResponse),
    Ticker(info::GetTickerResponse),
    Tickers(Vec<info::GetTickerResponse>),
}
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct RxResponse {
    // 订阅ID
    #[serde_as(as = "DisplayFromStr")]
    y: usize,
    #[serde(flatten)]
    data: Option<RxResponseData>,
    d: Option<RxResponseData>,
}

pub struct Ws {
    pub symbols: Vec<String>,
    pub ids: HashMap<usize, String>,
    pub book_mangers: HashMap<String, Mutex<DepthManger>>,
    pub books: HashMap<String, watch::Sender<Depth>>,
    pub index_prices: HashMap<String, watch::Sender<f64>>,
}
impl Ws {
    pub fn new(symbols: Vec<String>) -> Ws {
        let book_mangers = symbols.iter().map(|s| (s.clone(), Mutex::new(DepthManger::new()))).collect();
        let books = symbols.iter().map(|s| (s.clone(), watch::channel(Depth::default()).0)).collect();
        let index_prices = symbols.iter().map(|s| (s.clone(), watch::channel(0.0).0)).collect();
        let ids = symbols.iter().map(Clone::clone).enumerate().collect();
        Ws {
            symbols,
            ids,
            book_mangers,
            books,
            index_prices,
        }
    }
    pub fn clear(&self) {
        self.books.values().for_each(|x| {
            x.send_replace(Depth::default());
        });
    }
    pub async fn run(&self, tick: &[f64]) -> Result<(), anyhow::Error> {
        if tick.len() != self.symbols.len() {
            return Err(anyhow::anyhow!("price tick size error: {}", tick.len()));
        }
        let (ws_stream, _) = tokio_tungstenite::connect_async(HOST).await?;
        tracing::info!(base_url = HOST, "WebSocket connected");
        tokio::time::sleep(Duration::from_secs(1)).await;
        let (mut write, mut read) = ws_stream.split();
        for (i, s) in self.symbols.iter().enumerate() {
            let ch = format!("{s}_{}_25", tick[i]);
            let req_price = TxRequest {
                x: 3,
                y: 3000000001 + i,
                z: 1,
                e: r#"{"bvc":"202","isUsd":1}"#,
                a: Channel { i: ch },
            };
            write.send(Message::Text(serde_json::to_string(&req_price)?.into())).await?;
            let req_index = TxRequest {
                x: 1,
                y: 1000000001 + i,
                z: 1,
                e: r#"{"bvc":"202","isUsd":1}"#,
                a: Channel { i: s.clone() },
            };
            write.send(Message::Text(serde_json::to_string(&req_index)?.into())).await?;
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
            } else if let Message::Binary(bin) = message {
                bin.into()
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
            match m.data.or(m.d) {
                Some(RxResponseData::OrderBook(d)) => {
                    let Some(symbol) = self.ids.get(&(m.y - 3000000001)) else {
                        tracing::warn!("Not id {}", m.y);
                        continue;
                    };
                    if let Some(ch) = self.books.get(symbol) {
                        ch.send_replace(Depth {
                            bid: d.b.iter().map(|x| Order::new(x.0, x.1)).collect(),
                            ask: d.s.iter().map(|x| Order::new(x.0, x.1)).collect(),
                            version: (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as u64,
                        });
                    } else {
                        tracing::warn!("Not init {}", symbol);
                    }
                }
                Some(RxResponseData::Ticker(d)) => {
                    let symbol = d.a;
                    if let Some(ch) = self.index_prices.get(&symbol) {
                        ch.send_replace(d.d);
                    } else {
                        tracing::warn!("Not init {}", symbol);
                    }
                }
                Some(RxResponseData::Tickers(d)) => {
                    for x in d {
                        let symbol = x.a;
                        if let Some(ch) = self.index_prices.get(&symbol) {
                            ch.send_replace(x.d);
                        } else {
                            tracing::warn!("Not init {}", symbol);
                        }
                    }
                }
                None => {
                    tracing::warn!(text = ?text, "Unhandled message");
                }
            }
        }
    }
}
