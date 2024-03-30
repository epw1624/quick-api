use callframe::Callframe;
use clap::{command, Arg, Command};
use std::collections::HashMap;
use futures::executor::block_on;

mod callframe;

#[tokio::main]
async fn main() {

    let match_result = command!()
    .about("quick-api: A command line interface for prototyping API calls")
    .subcommand(Command::new("basic_auth")
        .arg(Arg::new("username")
            .short('u')
            .required(true))
        .arg(Arg::new("password")
            .short('p')
            .required(true))
    )
    .arg(Arg::new("params")
        .long("params")
        .value_delimiter(',')
        .help("Comma delimited series of key=value parameter pairs")
    )
    .arg(Arg::new("name")
        .short('n')
        .long("name")
        .help("The name of the API call, will use as filename when saving call")
        .required(true)
    ).arg(Arg::new("url")
        .long("url")
        .help("URL of the API call")
        .required(true)
    ).arg(Arg::new("method")
        .long("method")
        .help("HTTP method")
        .value_parser(["GET", "POST", "PUT", "DELETE"])
        .required(true)
    ).get_matches();

    let name: String = match_result.get_one::<String>("name").unwrap().to_string();
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
        name,
        url,
        method,
        headers: HashMap::new(),
        params: HashMap::new(),
        response: None,
        status: None
    };

    // Check if params are provided
    if let Some(params) = match_result.get_many::<String>("params") {
        let params_list: Vec<&String> = params.collect::<Vec<_>>();
        
        let mut params_map = HashMap::<String, String>::new();
        for param in params_list {
            let mut parts = param.split('=');

            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                params_map.insert(key.to_string(), value.to_string());
            }
        }
        callframe.params = params_map;
    }

    // Check if basic auth is provided
    if let Some(basic_auth) = match_result.subcommand_matches("basic_auth") {
        let username = basic_auth.get_one::<String>("username").unwrap().as_str();
        let password = basic_auth.get_one::<String>("password").unwrap().as_str();

        callframe.add_basic_auth(username, password);
    }

    let future = callframe.make_request();
    let _ = block_on(future);

    let _ = Callframe::save_callframe(&callframe);

}
