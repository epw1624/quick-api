use dialoguer::Select;

pub fn select_method() -> reqwest::Method {
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