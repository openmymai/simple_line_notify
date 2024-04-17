use reqwest::Response;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() {
    match send().await{
        Ok(_res) => (),
        Err(e) => println!("error: {}", e),
    }
}

async fn send() -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let token = match env::var("TOKEN") {
        Ok(token) => token,
        Err(e) => e.to_string(),
    };

    let url = "https://notify-api.line.me/api/notify";
    let line_message = "Cool! from Line notify";
    
    let mut message = HashMap::new();
    message.insert("message", line_message);

    let response = reqwest::Client::new()
        .post(url)
        .bearer_auth(token)
        .form(&message)
        .send()
        .await?;

    println!("Status is {:?}", response.status());

    Ok(response)
}