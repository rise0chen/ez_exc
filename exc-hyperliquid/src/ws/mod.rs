use exc_util::types::book::{Depth, Order};
use futures::StreamExt;
use hypersdk::hypercore::{self, ws::Event, Incoming, Subscription};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::watch;

#[derive(Clone)]
pub struct Ws {
    pub symbols: Vec<String>,
    pub funding_rates: Arc<HashMap<String, watch::Sender<f64>>>,
    pub index_prices: Arc<HashMap<String, watch::Sender<f64>>>,
    pub books: Arc<HashMap<String, watch::Sender<Depth>>>,
}
impl Ws {
    pub fn new(symbols: Vec<String>) -> Ws {
        let funding_rates = Arc::new(symbols.iter().map(|s| (s.clone(), watch::channel(0.0).0)).collect());
        let index_prices = Arc::new(symbols.iter().map(|s| (s.clone(), watch::channel(0.0).0)).collect());
        let books = Arc::new(symbols.iter().map(|s| (s.clone(), watch::channel(Depth::default()).0)).collect());
        Ws {
            symbols,
            funding_rates,
            index_prices,
            books,
        }
    }
    pub fn clear(&self) {
        self.funding_rates.values().for_each(|x| {
            x.send_replace(0.0);
        });
        self.index_prices.values().for_each(|x| {
            x.send_replace(0.0);
        });
        self.books.values().for_each(|x| {
            x.send_replace(Depth::default());
        });
    }
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let mut ws = hypercore::mainnet_ws();
        for coin in &self.symbols {
            ws.subscribe(Subscription::ActiveAssetCtx { coin: coin.into() });
            ws.subscribe(Subscription::L2Book { coin: coin.into() });
        }
        while let Some(event) = ws.next().await {
            let Event::Message(msg) = event else { continue };
            match msg {
                Incoming::ActiveAssetCtx { coin, ctx } => {
                    if let Some(ch) = self.funding_rates.get(&coin) {
                        ch.send_replace(ctx.funding.as_f64());
                    } else {
                        tracing::warn!("Not init {} funding_rate", coin);
                    }
                    if let Some(ch) = self.index_prices.get(&coin) {
                        ch.send_replace(ctx.oracle_px.unwrap_or_default().as_f64());
                    } else {
                        tracing::warn!("Not init {} index_price", coin);
                    }
                }
                Incoming::L2Book(book) => {
                    if let Some(ch) = self.books.get(&book.coin) {
                        let bid = book
                            .bids()
                            .iter()
                            .map(|x| Order {
                                price: x.px.as_f64(),
                                size: x.sz.as_f64(),
                            })
                            .collect();
                        let ask = book
                            .asks()
                            .iter()
                            .map(|x| Order {
                                price: x.px.as_f64(),
                                size: x.sz.as_f64(),
                            })
                            .collect();
                        ch.send_replace(Depth {
                            bid,
                            ask,
                            version: book.time,
                        });
                    } else {
                        tracing::warn!("Not init {} book", book.coin);
                    }
                }
                _ => {
                    tracing::warn!(message = ?msg, "Unhandled ws message");
                }
            }
        }
        Ok(())
    }
}
