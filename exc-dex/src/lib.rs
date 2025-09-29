//! Exc-dex: Dex exchange services.

extern crate tracing;

pub mod abi;
mod error;
pub mod key;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "rustls-tls", feature = "native-tls"))] {
        /// Exchange.
        //pub mod exchange;
        pub mod service;

        //pub use exchange::DexExchange;
    } else {
        compile_error!("Either feature 'rustls-tls' or 'native-tls' must be enabled");
    }
}
