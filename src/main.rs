extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::env;
use std::process;

use reqwest::{Client, Response, Error};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseObject {
    content: String,

    author: String,
}

const CATEGORIES: [&str; 63] = ["age", 
"athletics",
"business",
"change",
"character",
"competition",
"conservative",
"courage",
"creativity",
"education",
"ethics",
"failure",
"faith",
"family",
"film",
"freedom",
"friendship",
"future",
"generosity",
"genius",
"gratitude",
"happiness",
"health",
"history",
"honor",
"humor",
"humorous",
"imagination",
"inspirational",
"knowledge",
"leadership",
"life",
"literature",
"love",
"mathematics",
"motivational",
"nature",
"opportunity",
"pain",
"perseverance",
"philosophy",
"politics",
"proverb",
"religion",
"sadness",
"science",
"self",
"society",
"spirituality",
"sports",
"stupidity",
"success",
"technology",
"time",
"tolerance",
"truth",
"virtue",
"war",
"weakness",
"wellness",
"wisdom",
"work"];

#[async_std::main]
async fn main() {
    let acceptable_arguments = ["-c", "--category"];
    let args: Vec<String> = env::args().collect();

    let response_object: ResponseObject;

    if args.len() < 1 || args.len() > 3 {
        println!("Incorrect number of arguments.");
    }

    if args.len() == 1 {
        response_object = get_random_quote().await.json().await.unwrap();
    } else if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            print_help();
        }
        else if args[1] == "-c" || args[1] == "--category" {
            println!(
                "Incorrect usage of flags. Use -h or --help to learn more about flags you can use."
            );
        }
        process::exit(0);
    } else if args.len() == 3 {
        if !acceptable_arguments.contains(&args[1].as_str()) {
            println!(
                "Incorrect usage of flags. Use -h or --help to learn more about flags you can use."
            );
            process::exit(1);
        }

        response_object =  get_category_quote(args[2].as_str()).await.json().await.unwrap();

    } else {
        unreachable!();
    }

    println!(
        "\"{}\"\n{}",
        response_object.content, response_object.author
    );
}

async fn get_random_quote() -> Response {
    let response: Result<Response, reqwest::Error> = Client::new()
        .get("https://api.quotable.io/random?tags=love")
        .send()
        .await;

    return check_response(response);
}

async fn get_category_quote(category: &str) -> Response {
    if !CATEGORIES.contains(&category) {
        println!("Incorrect category. Use -h or --help to learn more about categories you can use.");
        process::exit(2);
    }
    let response: Result<Response, reqwest::Error> = Client::new()
        .get(format!("https://api.quotable.io/random?tags={}", category))
        .send()
        .await;


    return check_response(response);
}

fn check_response(response: Result<Response, reqwest::Error>) -> Response {
    let response = response.unwrap();
    if response.status() != 200 {
            println!("Couldn't fetch the quote for some reason. Check your internet connection.");
            process::exit(1);
    }

    return response; 
}

fn print_help() {
    let text = 
    "
    Usage: 
        cliquote
        cliquote -h 
        cliquote -c [category]

    Arguments: 
        -h --help        display this help and exit
        -c --category    (with category name)

    Categories:
        age,           athletics,   business,     change,
        character,     competition, conservative, courage,
        creativity,    education,   ethics,       failure,
        faith,         family,      film,         freedom,
        friendship,    future,      generosity,   genius,
        gratitude,     happiness,   health,       history,
        honor,         humor,       humorous,     imagination,
        inspirational, knowledge,   leadership,   life,
        literature,    love,        mathematics,  motivational,
        nature,        opportunity, pain,         perseverance,
        philosophy,    politics,    proverb,      religion,
        sadness,       science,     self,         society,
        spirituality,  sports,      stupidity,    success,
        technology,    time,        tolerance,    truth,
        virtue,        war,         weakness,     wellness,
        wisdom,        work";



    println!("{}", text);
}
