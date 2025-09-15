use exc_core::ExchangeError;
use exc_util::symbol::Symbol;
use exc_util::types::book::Depth;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct GetDepthRequest {
    pub symbol: Symbol,
    pub limit: u16,
    pub ch: Sender<Result<Depth, ExchangeError>>,
}
