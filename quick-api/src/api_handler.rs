use std::error::Error;
use serde_json;

pub async fn get(client: reqwest::Client, request_url: String) -> Result<serde_json::Value, Box<dyn Error>> {
    let response: String = client.get(&request_url).send().await?.text().await?;
    let v: serde_json::Value = serde_json::from_str(&response).unwrap();
    println!("{:?}", v);
    
    Ok(v)
}