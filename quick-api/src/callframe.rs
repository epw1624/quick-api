pub mod callframe_visitor;
pub mod new;
pub mod load;

use std::{fs, io};
use std::error::Error;
use std::collections::HashMap;
use serde::Deserializer;
use serde_json;
use serde::{ser::{Serialize, SerializeStruct}, Deserialize};
use base64::{Engine as _, engine::general_purpose};
use callframe_visitor::CallframeVisitor;

#[derive(Default)]
pub struct Callframe {
    pub name: String,
    pub url: String,
    pub method: reqwest::Method,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub status: Option<reqwest::StatusCode>,
    pub response: Option<serde_json::Value>,
}

impl Callframe { // Public functions

    pub async fn make_request(&mut self) -> Result<serde_json::Value, Box<dyn Error>> {
        // build url with params
        let mut full_url = self.url.clone();
        for (key, value) in &self.params {
            let param = format!("?{}={}", key, value);
            full_url.push_str(&param);
        }

        let client = reqwest::Client::new();
        let mut request: reqwest::RequestBuilder = client.request(self.method.clone(), &full_url);
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = match request.send().await {
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

    pub fn save_callframe(&self) -> io::Result<()> {
        let data_directory = "data";
        let file = fs::File::create(format!("{}/{}.json", data_directory, &self.name))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn add_basic_auth(&mut self, username: &str, password: &str) {
        let credentials = username.to_string() + ":" + password;
        let encoded_credentials = general_purpose::STANDARD.encode(credentials);
        let auth = "Basic ".to_owned() + encoded_credentials.as_str();

        self.headers.insert("Authorization".to_string(), auth);
    }

    pub fn add_params(&mut self, params: &str) {
        let params_list: Vec<&str> = params.split(',').collect();

        let mut params_map = HashMap::<String, String>::new();
        for param in params_list {
            let mut parts = param.split('=');

            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                params_map.insert(key.to_string(), value.to_string());
            }
        }
        self.params = params_map;
    }

    pub fn get_summary(&self) -> HashMap<&str, &str> {
        let mut summary: HashMap<&str, &str> = HashMap::new();

        summary.insert("Method", self.serialize_method());
        summary.insert("Name", self.name.as_str());
        summary.insert("URL", self.url.as_str());

        summary
    }

}

impl Callframe { // Private functions

    fn serialize_method(&self) -> &str {
        let method_as_string = match self.method {
            reqwest::Method::GET => "GET",
            reqwest::Method::POST => "POST",
            reqwest::Method::PUT => "PUT",
            reqwest::Method::DELETE => "DELETE",
            _ => "NULL"
        };
        method_as_string
    }
}

impl Serialize for Callframe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Callframe", 7)?;
        state.serialize_field("url", &self.url)?;

        state.serialize_field("method", self.serialize_method())?;

        state.serialize_field("headers", &self.headers)?;
        state.serialize_field("params", &self.params)?;

        let status_as_u16: u16 = match self.status {
            Some(status) => status.as_u16(),
            None => 0
        };

        state.serialize_field("status", &status_as_u16)?;
        state.serialize_field("response", &self.response)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Callframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = CallframeVisitor {};
        let result = deserializer.deserialize_map(visitor)?;
        Ok(result)
    }
}
