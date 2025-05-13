mod book;
mod info;
mod trading;

use crate::api::Request;
use exc_core::ExchangeError;
use futures::future::BoxFuture;
use tokio::sync::mpsc;
use tower::Service;

#[derive(Clone)]
pub struct Custom {
    tx: mpsc::Sender<Request>,
}

impl Custom {
    pub fn new() -> (Self, mpsc::Receiver<Request>) {
        let (tx, rx) = mpsc::channel(128);
        (Self { tx }, rx)
    }
}

impl Service<Request> for Custom {
    type Response = ();
    type Error = ExchangeError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let tx = self.tx.clone();
        Box::pin(async move { tx.send(req).await.map_err(|e| ExchangeError::Other(e.into())) })
    }
}
