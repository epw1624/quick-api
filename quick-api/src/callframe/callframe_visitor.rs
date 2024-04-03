use super::Callframe;
use serde::de::Visitor;
use reqwest::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

pub struct CallframeVisitor {}

impl<'de>Visitor<'de> for CallframeVisitor {
    type Value = Callframe;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize element")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>, 
    {
        let mut callframe = Callframe {
            name: String::default(),
            url: String::default(),
            method: reqwest::Method::default(),
            headers: HashMap::new(),
            params: HashMap::new(),
            status: None,
            response: None
        };

        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            match key.as_str() {
                "name" => callframe.name = value.as_str().unwrap_or_default().to_owned(),
                "url" => callframe.url = value.as_str().unwrap_or_default().to_owned(),
                "method" => callframe.method = deserialize_method(value).unwrap_or_default(),
                "headers" => callframe.headers = deserialize_map(value).unwrap_or_default(),
                "params" => callframe.params = deserialize_map(value).unwrap_or_default(),
                "status" => callframe.status = deserialize_status(value),
                "response" => callframe.response = Some(value),
                _ => println!("Unexpected field found in JSON")
            }
        }

        Ok(callframe)
    }
}

fn deserialize_map(value: Value) -> Result<HashMap<String, String>, serde_json::Error> {
        match value {
            Value::Object(map) => {
                let mut hashmap = HashMap::new();
                for (key, value) in map {
                    if let Value::String(v) = value {
                        hashmap.insert(key, v);
                    }
                }
                Ok(hashmap)
            }
            _ => Err(serde::de::Error::custom("Expected a JSON object"))
        }
    }

fn deserialize_method<'de>(value: Value) -> Result<Method, serde_json::Error> {
    match value.as_str() {
        Some("GET") => Ok(Method::GET),
        Some("POST") => Ok(Method::POST),
        Some("PUT") =>Ok(Method::PUT),
        Some("DELETE") => Ok(Method::DELETE),
        _ => Err(serde::de::Error::custom("Invalid HTTP method"))
    }
}

fn deserialize_status<'de>(value: Value) -> Option<StatusCode> {
    match value.as_u64() {
        Some(code) => {
            if code > u64::from(u16::MAX) {
                return None
            }
            else {
                return reqwest::StatusCode::from_u16(code as u16).ok()
            }
        }
        None => None

    }
}