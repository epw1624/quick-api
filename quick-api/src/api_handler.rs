use std::error::Error;
use serde_json;

pub async fn get(request_url: String) -> Result<(), Box<dyn Error>> {
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    let v: serde_json::Value = serde_json::from_str(&response).unwrap();
    println!("{:?}", v);
    
    Ok(())
}