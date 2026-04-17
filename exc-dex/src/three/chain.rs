use reqwest::Result;
use serde::Deserialize;

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
    pub rpc: Vec<String>,
    pub native_currency: NativeToken,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainData {
    pub chain: Chain,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseResult {
    pub data: ChainData,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub result: ResponseResult,
}

pub async fn get_chain(id: u64) -> Result<Chain> {
    let url = format!("https://chainid.network/page-data/chain/{id}/page-data.json");
    let resp = reqwest::get(url).await?;
    let data: Response = resp.json().await?;
    Ok(data.result.data.chain)
}
