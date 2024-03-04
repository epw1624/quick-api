use std::io;
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
    // temporarily, an array to index for selecting method, make nicer with a cli crate like clap later
    const METHODS: [reqwest::Method; 4] = [reqwest::Method::GET, reqwest::Method::POST, reqwest::Method::PUT, reqwest::Method::DELETE]; 
    pub fn build_callframe() -> Callframe {
        println!("Type or paste the request URL");
        let mut url = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");

        println!("Enter a number to select an HTTP request method\n
        1) GET\n
        2) POST\n
        3) PUT\n
        4) DELETE");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parsed_input: Result<i32, _> = input.trim().parse();
        match parsed_input {
            Ok(mut number) => {
                number -= 1;
                Callframe {
                    url,
                    method: Self::METHODS[number as usize].clone(),
                    headers: HashMap::new(),
                    response: None
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
    
    pub async fn make_request(&mut self) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.request(self.method.clone(), &self.url).send().await?.text().await?;
        let v: serde_json::Value = serde_json::from_str(&response).unwrap();
        self.response = Some(v.clone());
        println!("{:?}", self.response);

        Ok(v)
    }
}