use std::io;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use serde_json;
use serde::ser::{Serialize, SerializeStruct};

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

        Ok(v)
    }

    pub fn save_callframe(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

impl Serialize for Callframe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Callframe", 4)?;
        state.serialize_field("url", &self.url)?;

        let method_as_string = match self.method {
            reqwest::Method::GET => "GET",
            reqwest::Method::POST => "POST",
            reqwest::Method::PUT => "PUT",
            reqwest::Method::DELETE => "DELETE",
            _ => "NULL"
        };
        state.serialize_field("method", method_as_string)?;

        state.serialize_field("headers", &self.headers)?;
        state.serialize_field("response", &self.response)?;

        state.end()
    }
}

