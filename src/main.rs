extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;
use serde::Deserialize;

use reqwest::{Client, Error};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseObject {
    content: String,
    author: String,
}


#[async_std::main]
async fn main() {
    let acceptable_arguments = ["-c", "--category"];
    let args: Vec<String> = env::args().collect();

    let mut response_object: ResponseObject;

    if args.len() != 1 && args.len() != 3 {
        println!("Incorrect number of arguments.");
    }


    match args.len() {
        1 => response_object = random_quote().await.unwrap(),
        _ => unreachable!()
    };

    println!("\"{}\"\n{}", response_object.content, response_object.author);
}

async fn random_quote() -> Result<ResponseObject, Error> {
    let response: ResponseObject = Client::new().get("https://api.quotable.io/random").send().await?.json().await?;


    Ok(response)
}
