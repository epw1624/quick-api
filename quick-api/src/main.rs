use std::io;
use std::collections::HashMap;
use futures::executor::block_on;

mod api_handler;
mod callframe;

#[tokio::main]
async fn main() {
    println!("quick-api: A command line interface for prototyping API calls");

    loop {
        println!("Type or paste a URL to make an API call");

        let mut url = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");

        let mut callframe = callframe::Callframe {
            url,
            method: reqwest::Method::GET,
            headers: HashMap::new(),
            response: None, 
        };

        let future = callframe.make_request();
        let _ = block_on(future);
    }
}
