use std::error::Error;

pub async fn get(request_url: String) -> Result<(), Box<dyn Error>> {
    println!("{}", request_url);
    let response = reqwest::get(&request_url)
        .await?
        .text()
        .await?;
    println!("{}", response);
    
    Ok(())
}