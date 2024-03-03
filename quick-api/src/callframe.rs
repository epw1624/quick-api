use std::collections::HashMap;
use serde_json;

enum HTTP_Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

struct Callframe {
    url: string,
    method: HTTP_Method,
    headers: HashMap<String, String>,
    result: serde_json::Value,
}

impl Callframe {
    
}