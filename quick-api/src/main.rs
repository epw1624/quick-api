use std::io;
use futures::executor::block_on;

mod api_handler;

#[tokio::main]
async fn main() {
    println!("quick-api: A command line interface for prototyping API calls");

    loop {
        println!("Press any key to make test API call");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        const REQUEST_URL: &str = "https://api.sampleapis.com/beers/ale";
        let future = api_handler::get(REQUEST_URL.to_string());
        let _ = block_on(future);
    }
}
