
use reqwest::header;
use std::env;

use serde::{Serialize, Deserialize};
use spinners::{Spinner, Spinners};
use colored::Colorize;

const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/completions";
// const OPENAI_COMPLETIONS_URL: &str = "http://localhost:8889";

// API request struct
#[derive(Debug, Serialize, Deserialize)]
struct Request {
    model: String,
    prompt: String,
    max_tokens: u32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
}

// API response structs
#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<ResponseChoice>,
}

#[derive(Debug, Deserialize)]
struct ResponseChoice {
    text: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the api key from env
    let mut api_key = String::new();
    match env::var("OPENAI_API_KEY") {
        Ok(val) => {
            api_key.push_str(&val);
        }
        Err(_e) => {
            println!("{}", "OPENAI_API_KEY env not found".red());
            return Ok(());
        }
    }

    // collect args ane merge args into query
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <query>", args[0]);
        return Ok(());
    }
    args.remove(0);
    let query = args.join(" ");

    // build the request to API
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let auth_header = format!("Bearer {}", api_key);
    headers.insert("Authorization", auth_header.parse().unwrap());

    let p = format!("Question:\n{}\nAnswer:", query);
    let req = Request {
        model: "text-davinci-003".to_string(),
        prompt: p,
        max_tokens: 256, // TODO: allow to configure this via args
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
    };
    // serialize to json for post request
    let req_res = serde_json::to_string(&req);
    let mut req_json = String::new();
    match req_res {
        Ok(_val) => {
            req_json.push_str(&_val);
        }
        Err(_e) => {
            println!("{}", "Failed to serialize request".red());
            return Ok(());
        }
    }


    let client = reqwest::blocking::Client::new();

    // start loading 
    let loading_str = format!("{}", "Thinking...".green());
    let mut sp = Spinner::new(Spinners::Dots12, loading_str.into());

    // do request
    // TODO: proper handle errors
    let res = client.post(OPENAI_COMPLETIONS_URL)
        .headers(headers)
        .body(req_json)
        .send()?
        .text()?;

    // stop loading
    sp.stop_with_message("".to_string());

    // parse the response
    let result: Result<Response, serde_json::Error> = serde_json::from_str(&res.to_string());
    match result {
        Ok(r) => {
            if r.choices.len() == 0 {
                println!("{}", "No results".red());
            }
            println!("{}", r.choices[0].text);
        },
        Err(err) => {
            // Handle the error
            println!("{} {}", "Error deserializing response:".red(), err);
        }
    }

    Ok(())
}
