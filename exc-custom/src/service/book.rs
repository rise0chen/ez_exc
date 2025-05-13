use super::Custom;
use crate::api::book::GetDepthRequest;
use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use tokio::sync::oneshot;
use tower::ServiceExt;

impl Custom {
    pub async fn get_depth(&mut self, symbol: &Symbol, limit: u16) -> Result<Depth, ExchangeError> {
        let (tx, rx) = oneshot::channel();
        let req = GetDepthRequest {
            symbol: symbol.clone(),
            limit,
            ch: tx,
        };
        self.oneshot(req.into()).await?;
        rx.await.map_err(|e| ExchangeError::Other(e.into()))?
    }
}
