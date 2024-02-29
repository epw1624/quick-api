use std::io;

mod api_handler;

fn main() {
    println!("quick-api: A command line interface for prototyping API calls");

    loop {
        println!("Press any key to make test API call");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        const request_url: &str = "https://api.sampleapis.com/beers/ale";
        api_handler::get(request_url.to_string());
    }
}
