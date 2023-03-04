// Author: Kai Kousa
// License: MIT
// Description: CLI for OpenAI API
// Dependencies: reqwest, serde, serde_json, confy, dirs, spinners, colored
// Usage: kai <query>
// Example: kai "What is the meaning of life?"
//
// config file should be found at:
// MacOS: "/Users/user/Library/Application Support/rs.kai/kai.toml"
//
//
//
use reqwest::header;
use std::env;


use serde::{Serialize, Deserialize};
use spinners::{Spinner, Spinners};
use colored::Colorize;


const APP_NAME: &str = "kai";
const CONFIG_NAME: &str = "config";
const OPENAI_COMPLETIONS_URL: &str = "https://api.openai.com/v1/completions";
// const OPENAI_COMPLETIONS_URL: &str = "http://localhost:8889";

// config file 
#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
}

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
    // get the api key from config
    let cfg_result = confy::load(APP_NAME, CONFIG_NAME);
    let cfg:Config = match cfg_result {
        Ok(file) => file,
        Err(error) => {
            println!("{} {}", "Config error:".red(), error);
            return Ok(());
        }
    };
    // check api key exists
    if cfg.api_key == "" {
        println!("\n\nHello! this is <kai>, simple OpenAI GPT CLI client");
        // print config file path
        let config_path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME);
        println!("Please set up your OpenAI API key in the configuration file at \n\n\"{}\"", config_path.unwrap().to_str().unwrap().green());
        println!("\n\nYour keys can be found at https://platform.openai.com/account/api-keys");
        return Ok(());
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
    let auth_header = format!("Bearer {}", cfg.api_key);
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
