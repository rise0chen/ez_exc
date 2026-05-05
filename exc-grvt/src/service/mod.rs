mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use grvt_rust_sdk::{Environment, GrvtClient, GrvtConfig};

/// Grvt API.
#[derive(Clone)]
pub struct Grvt {
    key: Key,
    http: GrvtClient,
}
impl Grvt {
    pub async fn new(key: Key) -> Self {
        let config = GrvtConfig {
            environment: Environment::Prod,
            api_key: key.api_key.to_string(),
            sub_account_id: key.account_id.to_string(),
            private_key_hex: Some(key.secret_key.to_string()),
            chain_id: None,
        };
        let http = GrvtClient::from_config(&config).await.unwrap();

        Self { key, http }
    }
}
