//! Exc-hyperliquid: Hyperliquid exchange services.

extern crate tracing;

pub mod key;
pub mod symnol;
pub mod ws;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "rustls-tls", feature = "native-tls"))] {
        /// Exchange.
        //pub mod exchange;
        pub mod service;

        //pub use exchange::HyperliquidExchange;
    } else {
        compile_error!("Either feature 'rustls-tls' or 'native-tls' must be enabled");
    }
}
