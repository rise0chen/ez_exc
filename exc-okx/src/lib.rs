//! Exc-okx: Okx exchange services.

#[macro_use]
extern crate tracing;

pub mod api;
pub mod key;
pub mod response;
pub mod symnol;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "rustls-tls", feature = "native-tls"))] {
        /// Exchange.
        //pub mod exchange;
        pub mod service;

        //pub use exchange::OkxExchange;
    } else {
        compile_error!("Either feature 'rustls-tls' or 'native-tls' must be enabled");
    }
}
