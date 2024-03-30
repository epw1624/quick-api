use std::io;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use serde_json;
use serde::ser::{Serialize, SerializeStruct};
use base64::{Engine as _, engine::general_purpose};

pub struct Callframe {
    pub url: String,
    pub method: reqwest::Method,
    pub headers: HashMap<String, String>,
    pub status: Option<reqwest::StatusCode>,
    pub response: Option<serde_json::Value>,
}

impl Callframe {
    
    pub async fn make_request(&mut self) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = match client.request(self.method.clone(), &self.url).send().await {
            Ok(response) => response,
            Err(err) => return Err(Box::new(err)),
        };

        self.status = Some(response.status().clone());

        if !response.status().is_success() {
            return Err(format!("Request failed with status code: {}", response.status()).into());
        }

        let response_body = match response.text().await {
            Ok(body) => body,
            Err(err) => return Err(Box::new(err))
        };
        let v: serde_json::Value = serde_json::from_str(&response_body).unwrap();
        self.response = Some(v.clone());

        Ok(v)
    }

    pub fn save_callframe(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn add_basic_auth(&mut self, username: &str, password: &str) {
        let credentials = username.to_string() + ":" + password;
        let encoded_credentials = general_purpose::STANDARD.encode(credentials);
        let auth = "Basic ".to_owned() + encoded_credentials.as_str();

        self.headers.insert("Authorization".to_string(), auth);
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

        let status_as_u16: u16 = match self.status {
            Some(status) => status.as_u16(),
            None => 0
        };

        state.serialize_field("status", &status_as_u16)?;
        state.serialize_field("response", &self.response)?;

        state.end()
    }
}

