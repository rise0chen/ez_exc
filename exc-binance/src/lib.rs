//! Exc-binance: Binance exchange services.

pub mod futures_api;
pub mod key;
pub mod response;
pub mod spot_api;
pub mod symnol;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "rustls-tls", feature = "native-tls"))] {
        /// Exchange.
        //pub mod exchange;
        pub mod service;

        //pub use exchange::BinanceExchange;
    } else {
        compile_error!("Either feature 'rustls-tls' or 'native-tls' must be enabled");
    }
}
