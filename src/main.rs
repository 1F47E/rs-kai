
use reqwest::header;
use std::env;

use serde::{Deserialize};
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct TextCompletion {
    // id: String,
    // object: String,
    // created: u64,
    // model: String,
    choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    // index: u32,
    // logprobs: Option<Vec<f64>>,
    // finish_reason: String,
}

// #[derive(Debug, Deserialize)]
// struct Usage {
//     prompt_tokens: u32,
//     completion_tokens: u32,
//     total_tokens: u32,
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the api key
    let mut api_key = String::new();
    match env::var("OPENAI_API_KEY") {
        Ok(val) => {
            api_key.push_str(&val);
        }
        Err(_e) => {
            println!("OPENAI_API_KEY env not found");
            return Ok(());
        }
    }
    let args: Vec<String> = env::args().collect();

    // check if we have args
    if args.len() < 2 {
        println!("Usage: {} <query>", args[0]);
        return Ok(());
    }
    let query = &args[1];

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let auth_header = format!("Bearer {}", api_key);
    headers.insert("Authorization", auth_header.parse().unwrap());

    // let prompt = "Explain this: {QUERY} Answer:".replace("{QUERY}", query);
    // make a json object out or this body query string
    // but replace QUERY with a variable query
    let payload = r#"{
  "model": "text-davinci-003",
  "prompt": "Question:\n{PROMPT}\nAnswer:",
  "max_tokens": 64,
  "top_p": 1.0,
  "frequency_penalty": 0.0,
  "presence_penalty": 0.0
}"#.replace("{PROMPT}", &query);


    let client = reqwest::blocking::Client::new();

    // start loading animation
    let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());

    // do request
    let res = client.post("https://api.openai.com/v1/completions")
    // let res = client.post("http://localhost:8889")
        .headers(headers)
        .body(payload)
        .send()?
        .text()?;

    // stop the animation
    sp.stop_with_message("".to_string());
    // sp.stop();

    let result: TextCompletion = serde_json::from_str(&res.to_string()).unwrap();
    // check if we have a result
    if result.choices.len() < 1 {
        println!("No result");
        return Ok(());
    }
    let answer = result.choices[0].text.clone();
    // answer = answer.replace(" \n", "");
    println!("{}", answer);

    Ok(())
}
