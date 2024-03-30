use dialoguer::{Select, Input, Password, Confirm};
use super::Callframe;
use std::collections::HashMap;

pub fn new_callframe() -> Callframe {
    let name: String = Input::new()
        .with_prompt("Instance name")
        .interact_text()
        .unwrap();

    let url: String = Input::new()
        .with_prompt("URL")
        .interact_text()
        .unwrap();

    let method = select_method();

    let mut callframe = Callframe {
        name,
        url,
        method,
        headers: HashMap::new(),
        params: HashMap::new(),
        response: None,
        status: None
    };

    let basic_auth: bool = Confirm::new()
        .with_prompt("Does this call require Basic Authentication?")
        .interact()
        .unwrap();

    if basic_auth {
        let username: String = Input::new()
            .with_prompt("Username")
            .interact_text()
            .unwrap();

        let password: String = Password::new()
            .with_prompt("Password")
            .interact()
            .unwrap();

        callframe.add_basic_auth(username.as_str(), password.as_str());
    }

    let use_params: bool = Confirm::new()
        .with_prompt("Add parameters?")
        .interact()
        .unwrap();

    if use_params {
        let params: String = Input::new()
        .with_prompt("Enter comma delimited params in format <key>=<value>")
        .interact_text()
        .unwrap();

        callframe.add_params(params.as_str()); 
    }

    callframe
}

fn select_method() -> reqwest::Method {
    let methods = vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
    ];

    let selection = Select::new()
        .with_prompt("Choose an HTTP method")
        .items(&methods)
        .default(0) // Optionally, you can set a default selection
        .interact()
        .unwrap();

    let method = match selection {
        0 => reqwest::Method::GET,
        1 => reqwest::Method::POST,
        2 => reqwest::Method::PUT,
        3 => reqwest::Method::DELETE,
        _ => reqwest::Method::GET
    };
    method
}