use std::io;
use futures::executor::block_on;

mod api_handler;

#[tokio::main]
async fn main() {
    println!("quick-api: A command line interface for prototyping API calls");

    loop {
        println!("Type or paste a URL to make an API call");

        let mut url = String::new();

        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");

        let client: reqwest::Client = reqwest::Client::new();

        let future = api_handler::get(client, url);
        let _ = block_on(future);
    }
}
