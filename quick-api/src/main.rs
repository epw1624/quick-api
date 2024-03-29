use callframe::Callframe;
use clap::{command, Arg};
use std::collections::HashMap;
use futures::executor::block_on;

mod callframe;

#[tokio::main]
async fn main() {

    let match_result = command!()
    .about("quick-api: A command line interface for prototyping API calls")
    .arg(Arg::new("url")
        .long("url")
        .help("URL of the API call")
        .required(true)
    ).arg(Arg::new("method")
        .long("method")
        .help("HTTP method")
        .value_parser(["GET", "POST", "PUT", "DELETE"])
        .required(true)
    ).get_matches();

    let url: String = match_result.get_one::<String>("url").unwrap().to_string();
    let method_string: String = match_result.get_one::<String>("method").unwrap().to_string();

    let method = match method_string.as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET // this should never happen because of value_parser() but I needed to include a default case
    };

    let mut callframe = Callframe {
        url,
        method,
        headers: HashMap::new(),
        response: None
    };

    let future = callframe.make_request();
    let _ = block_on(future);

    let _ = Callframe::save_callframe(&callframe, "test_output.json");

}
