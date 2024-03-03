use std::collections::HashMap;
use serde_json;

struct Callframe {
    url: string,
    method: reqwest::Method,
    headers: HashMap<String, String>,
    result: serde_json::Value,
}

impl Callframe {
    fn make_request(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.request(self.method, self.url).send().await?.text().await?;
        let v = serde_json::from_str(&response).unwrap();
        println!("{:?}", v);
    
        Ok(v)
    }
}