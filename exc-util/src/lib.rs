pub mod asset;
pub mod constant;
pub mod error;
pub mod http;
pub mod interface;
pub mod symbol;
pub mod traits;
pub mod types;

pub fn init() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}
