pub mod info;

use core::time::Duration;
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, sync::Arc};
use time::OffsetDateTime;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::Message;

const HOST: &str = "wss://fapi.bitunix.com/public/";
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RequestArg {
    pub ch: &'static str,
    pub symbol: String,
}
#[derive(Debug, Serialize)]
pub struct Request {
    pub op: &'static str,
    pub args: Vec<RequestArg>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "ch", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum ResponseData {
    Price(info::GetIndexPriceResponse),
}
#[derive(Debug, Deserialize)]
pub struct Response {
    pub symbol: String,
    pub ts: u64,
    #[serde(flatten)]
    pub data: ResponseData,
}

#[derive(Clone)]
pub struct Ws {
    pub symbols: Vec<String>,
    pub index_prices: Arc<HashMap<String, watch::Sender<f64>>>,
}
impl Ws {
    pub fn new(symbols: Vec<String>) -> Ws {
        let index_prices = Arc::new(symbols.iter().map(|s| (s.clone(), watch::channel(0.0).0)).collect());
        Ws { symbols, index_prices }
    }
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(HOST).await?;
        tracing::info!(base_url = HOST, "WebSocket connected");
        let (mut write, mut read) = ws_stream.split();

        let req_price = Request {
            op: "subscribe",
            args: self
                .symbols
                .iter()
                .map(|s| RequestArg {
                    ch: "price",
                    symbol: s.clone(),
                })
                .collect(),
        };
        write.send(Message::Text(serde_json::to_string(&req_price)?.into())).await?;
        // Message handling loop
        let mut last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
        let mut interval = tokio::time::interval(Duration::from_secs(28));
        loop {
            let message = match futures_util::future::select(Box::pin(interval.tick()), read.next()).await {
                futures_util::future::Either::Left(_) => {
                    if OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000 - last_time > 120 {
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

            if let Message::Text(text) = message {
                let m: Response = match serde_json::from_slice(text.as_bytes()) {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::warn!(e = ?e, text = ?text, "Unhandled message");
                        continue;
                    }
                };
                match m.data {
                    ResponseData::Price(d) => {
                        if let Some(ch) = self.index_prices.get(&m.symbol) {
                            ch.send_replace(d.ip);
                        } else {
                            tracing::warn!("Not init {}", m.symbol);
                        }
                    }
                }
            } else if let Message::Pong(_) = message {
                last_time = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000_000;
            } else {
                tracing::warn!(message = ?message, "Unhandled ws message");
            }
        }
    }
}
