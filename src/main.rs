extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::env;
use std::process;

use reqwest::{Client, Response};

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

    let response_object: ResponseObject;

    if args.len() < 1 || args.len() > 3 {
        println!("Incorrect number of arguments.");
    }

    if args.len() == 1 {
        response_object = random_quote().await.json().await.unwrap();
    } else if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--here" {
            print_help();
        }
    } else if args.len() == 3 {
        if !acceptable_arguments.contains(&args[1].as_str()) {
            println!(
                "Incorrect usage of flags. Use -h or --help to learn more about flags you can use."
            );
            process::exit(1);
        }
    } else {
        unreachable!();
    }

    // println!(
    //     "\"{}\"\n{}",
    //     response_object.content, response_object.author
    // );
}

async fn random_quote() -> Response {
    let response: Result<Response, reqwest::Error> = Client::new()
        .get("https://api.qotable.io/random")
        .send()
        .await;

    if response.is_err() {
        println!("Couldn't fetch the quote for some reason. Check your internet connection.");
        process::exit(1);
    }

    response.unwrap()
}

fn print_help() {
    println!("in help function");
}
