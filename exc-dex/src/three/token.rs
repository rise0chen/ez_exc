use reqwest::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub current_price: f64,
}

pub async fn get_token(token: &str) -> Result<Option<TokenData>> {
    let url = format!("https://api.coingecko.com/api/v3/coins/markets?x_cg_demo_api_key=CG-6pa5KznkcP8yJekrrc4dbjK7&vs_currency=usd&symbols={token}");
    let resp = reqwest::get(url).await?;
    let mut data: Vec<TokenData> = resp.json().await?;
    Ok(data.pop())
}
