extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use reqwest::Error;

#[async_std::main]
async fn main() {
    random_quote().await;

}

async fn random_quote() -> Result<(), Error> {
    let response = reqwest::get("https://api.quotable.io/random").await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())    
}
