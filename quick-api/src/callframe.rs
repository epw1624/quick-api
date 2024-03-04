use std::error::Error;
use std::collections::HashMap;
use serde_json;

pub struct Callframe {
    pub url: String,
    pub method: reqwest::Method,
    pub headers: HashMap<String, String>,
    pub response: Option<serde_json::Value>,
}

impl Callframe {
    pub async fn make_request(&mut self) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.request(self.method.clone(), &self.url).send().await?.text().await?;
        let v: serde_json::Value = serde_json::from_str(&response).unwrap();
        self.response = Some(v.clone());
        println!("{:?}", self.response);

        Ok(v)
    }
}