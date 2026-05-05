mod account;
mod book;
mod earn;
mod info;
mod trading;

use crate::key::Key;
use paradex::rest::Client;

/// Paradex API.
#[derive(Clone)]
pub struct Paradex {
    key: Key,
    http: Client,
}
impl Paradex {
    pub async fn new(key: Key) -> Self {
        let url = paradex::url::URL::Production;
        let mut http = Client::new(url, Some(key.secret_key.to_string())).await.unwrap();
        http.interactive = !key.pro;

        Self { key, http }
    }
}
