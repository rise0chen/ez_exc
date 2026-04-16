use reqwest::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rpc {
    pub url: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeToken {
    pub symbol: String,
    pub decimals: i32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub chain: String,
    pub rpc: Vec<Rpc>,
    pub native_currency: NativeToken,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainData {
    pub chain: Chain,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub page_props: ChainData,
}

pub async fn get_chain(id: u64) -> Result<Chain> {
    let url = format!("https://chainlist.org/_next/data/vKZl0OZPblRJq2J9xzw4h/chain/{id}.json");
    let resp = reqwest::get(url).await?;
    let data: Response = resp.json().await?;
    Ok(data.page_props.chain)
}
